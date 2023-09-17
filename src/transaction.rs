// allow dead code
#![allow(dead_code)]
// allow dead imports
#![allow(unused_imports)]
// allow unused var
#![allow(unused_variables)]
// no builtins
use core::mem::{MaybeUninit, self};

use crate::api::*;
use crate::{c, hook_account, ledger_seq, AccountId, AccountType, AmountType, TxnType};

/// Builds a transaction to send XRP.
/// Equivalent to PREPARE_PAYMENT_SIMPLE in `macro.h` in
/// official hooks API.
pub struct XrpPaymentBuilder<'a> {
    drops: u64,
    to_address: &'a [u8; 20],
    dest_tag: u32,
    src_tag: u32,
}

#[repr(u8)]
enum FieldCode {
    TransactionType = 0x12,
    Flags = 0x2,
    SourceTag = 0x3,
    Sequence = 0x4,
    DestinationTag = 0xE,
    FirstLedgerSequence = 0x1A,
    LastLedgerSequence = 0x1B,
}

/// Builds a transaction.
pub trait TransactionBuilder<const TXN_LEN: usize> {
    /// Byte length of the transaction.
    const TXN_LEN: usize = TXN_LEN;
    /// Transaction type of the transaction.
    const TXN_TYPE: TxnType;
    /// Builds a specific transaction.
    fn build(self) -> Result<[u8; TXN_LEN]>;
}

/// A buffer for building a transaction.
pub struct TransactionBuffer<const TXN_LEN: usize> {
    buf: [MaybeUninit<u8>; TXN_LEN],
    pos: usize,
}

// Ugly to specify the length of the transaction here instead
// of declaring it as an associated constant, but specifying
// constant has the return type in `build` method is unstable
// in Rust nightly right now. See `generic_const_exprs` feature.
impl<const TXN_LEN: usize> TransactionBuffer<TXN_LEN> {
    /// Encodes a transaction type.
    #[inline(always)]
    pub fn encode_txn_type(&mut self, tt: TxnType) {
        unsafe {
            self.buf
                .get_unchecked_mut(self.pos)
                .as_mut_ptr()
                .write_volatile(tt.into());
            self.buf
                .get_unchecked_mut(self.pos + 1)
                .as_mut_ptr()
                .write_volatile(((tt as u16 >> 8) & 0xFF) as u8);
            self.buf
                .get_unchecked_mut(self.pos + 2)
                .as_mut_ptr()
                .write_volatile((tt as u16 & 0xFF) as u8);
        }
        self.pos += 3;
    }

    /// Encodes a u32 value.
    #[inline(always)]
    pub fn encode_u32(&mut self, data: u32, field: u8) {
        // self.buf[self.pos] = 0x20 + (field & 0x0F);
        // self.buf[self.pos + 1] = ((data >> 24) & 0xFF) as u8;
        // self.buf[self.pos + 2] = ((data >> 16) & 0xFF) as u8;
        // self.buf[self.pos + 3] = ((data >> 8) & 0xFF) as u8;
        // self.buf[self.pos + 4] = (data & 0xFF) as u8;
        unsafe {
            self.buf
                .get_unchecked_mut(self.pos)
                .as_mut_ptr()
                .write_volatile(0x20 + (field & 0x0F));
            self.buf
                .get_unchecked_mut(self.pos + 1)
                .as_mut_ptr()
                .write_volatile(((data >> 24) & 0xFF) as u8);
            self.buf
                .get_unchecked_mut(self.pos + 2)
                .as_mut_ptr()
                .write_volatile(((data >> 16) & 0xFF) as u8);
            self.buf
                .get_unchecked_mut(self.pos + 3)
                .as_mut_ptr()
                .write_volatile(((data >> 8) & 0xFF) as u8);
            self.buf
                .get_unchecked_mut(self.pos + 4)
                .as_mut_ptr()
                .write_volatile((data & 0xFF) as u8);
        }
        self.pos += 5;
    }

    /// Encodes a u32 value with a field id.
    #[inline(always)]
    pub fn encode_u32_with_field_id(&mut self, data: u32, field: u8) {
        // self.buf[self.pos] = 0x20;
        // self.buf[self.pos + 1] = field;
        // self.buf[self.pos + 2] = ((data >> 24) & 0xFF) as u8;
        // self.buf[self.pos + 3] = ((data >> 16) & 0xFF) as u8;
        // self.buf[self.pos + 4] = ((data >> 8) & 0xFF) as u8;
        // self.buf[self.pos + 5] = (data & 0xFF) as u8;
        unsafe {
            self.buf
                .get_unchecked_mut(self.pos)
                .as_mut_ptr()
                .write_volatile(0x20);
            self.buf
                .get_unchecked_mut(self.pos + 3)
                .as_mut_ptr()
                .write_volatile(((data >> 16) & 0xFF) as u8);
            self.buf
                .get_unchecked_mut(self.pos + 1)
                .as_mut_ptr()
                .write_volatile(field);
            self.buf
                .get_unchecked_mut(self.pos + 4)
                .as_mut_ptr()
                .write_volatile(((data >> 8) & 0xFF) as u8);
            self.buf
                .get_unchecked_mut(self.pos + 2)
                .as_mut_ptr()
                .write_volatile(((data >> 24) & 0xFF) as u8);
            self.buf
                .get_unchecked_mut(self.pos + 5)
                .as_mut_ptr()
                .write_volatile((data & 0xFF) as u8);
        }
        self.pos += 6;
    }

    /// Encodes amount in drops.
    #[inline(always)]
    pub fn encode_drops(&mut self, drops: u64, amount_type: AmountType) {
        self.encode_drops_at(self.pos, drops, amount_type);
    }

    /// Encodes amount in drops at a specific position.
    #[inline(always)]
    pub fn encode_drops_at(&mut self, pos: usize, drops: u64, amount_type: AmountType) {
        let amount_type: u8 = amount_type.into();
        // self.buf[pos] = 0x60 + (amount_type & 0x0F);
        // self.buf[pos + 1] = (0b01000000 + ((drops >> 56) & 0b00111111)) as u8;
        // self.buf[pos + 2] = ((drops >> 48) & 0xFF) as u8;
        // self.buf[pos + 3] = ((drops >> 40) & 0xFF) as u8;
        // self.buf[pos + 4] = ((drops >> 32) & 0xFF) as u8;
        // self.buf[pos + 5] = ((drops >> 24) & 0xFF) as u8;
        // self.buf[pos + 6] = ((drops >> 16) & 0xFF) as u8;
        // self.buf[pos + 7] = ((drops >> 8) & 0xFF) as u8;
        // self.buf[pos + 8] = (drops & 0xFF) as u8;
        unsafe {
            self.buf
                .get_unchecked_mut(pos)
                .as_mut_ptr()
                .write_volatile(0x60 + (amount_type & 0x0F));
            self.buf
                .get_unchecked_mut(pos + 1)
                .as_mut_ptr()
                .write_volatile((0b01000000 + ((drops >> 56) & 0b00111111)) as u8);
            self.buf
                .get_unchecked_mut(pos + 2)
                .as_mut_ptr()
                .write_volatile(((drops >> 48) & 0xFF) as u8);
            self.buf
                .get_unchecked_mut(pos + 3)
                .as_mut_ptr()
                .write_volatile(((drops >> 40) & 0xFF) as u8);
            self.buf
                .get_unchecked_mut(pos + 4)
                .as_mut_ptr()
                .write_volatile(((drops >> 32) & 0xFF) as u8);
            self.buf
                .get_unchecked_mut(pos + 5)
                .as_mut_ptr()
                .write_volatile(((drops >> 24) & 0xFF) as u8);
            self.buf
                .get_unchecked_mut(pos + 6)
                .as_mut_ptr()
                .write_volatile(((drops >> 16) & 0xFF) as u8);
            self.buf
                .get_unchecked_mut(pos + 7)
                .as_mut_ptr()
                .write_volatile(((drops >> 8) & 0xFF) as u8);
            self.buf
                .get_unchecked_mut(pos + 8)
                .as_mut_ptr()
                .write_volatile((drops & 0xFF) as u8);
        }
        self.pos += 9;
    }

    /// Encodes a signing public key as null.
    #[inline(always)]
    pub fn encode_signing_pubkey_as_null(&mut self) {
        // self.buf[self.pos] = 0x73;
        // self.buf[self.pos + 1] = 0x21;
        // leave self.buf[self.pos + 2..self.pos + 35] as 0 because they
        // are already initialized to 0 and meant to be like that to
        // represent null
        unsafe {
            self.buf
                .get_unchecked_mut(self.pos)
                .as_mut_ptr()
                .write_volatile(0x73);
            self.buf
                .get_unchecked_mut(self.pos + 1)
                .as_mut_ptr()
                .write_volatile(0x21);

            // avoid creating loops in the resulting wasm
            self.buf
                .get_unchecked_mut(self.pos + 2)
                .as_mut_ptr()
                .write_volatile(0);
            self.buf
                .get_unchecked_mut(self.pos + 3)
                .as_mut_ptr()
                .write_volatile(0);
            self.buf
                .get_unchecked_mut(self.pos + 4)
                .as_mut_ptr()
                .write_volatile(0);
            self.buf
                .get_unchecked_mut(self.pos + 5)
                .as_mut_ptr()
                .write_volatile(0);
            self.buf
                .get_unchecked_mut(self.pos + 6)
                .as_mut_ptr()
                .write_volatile(0);
            self.buf
                .get_unchecked_mut(self.pos + 7)
                .as_mut_ptr()
                .write_volatile(0);
            self.buf
                .get_unchecked_mut(self.pos + 8)
                .as_mut_ptr()
                .write_volatile(0);
            self.buf
                .get_unchecked_mut(self.pos + 9)
                .as_mut_ptr()
                .write_volatile(0);
            self.buf
                .get_unchecked_mut(self.pos + 10)
                .as_mut_ptr()
                .write_volatile(0);
            self.buf
                .get_unchecked_mut(self.pos + 11)
                .as_mut_ptr()
                .write_volatile(0);
            self.buf
                .get_unchecked_mut(self.pos + 12)
                .as_mut_ptr()
                .write_volatile(0);
            self.buf
                .get_unchecked_mut(self.pos + 13)
                .as_mut_ptr()
                .write_volatile(0);
            self.buf
                .get_unchecked_mut(self.pos + 14)
                .as_mut_ptr()
                .write_volatile(0);
            self.buf
                .get_unchecked_mut(self.pos + 15)
                .as_mut_ptr()
                .write_volatile(0);
            self.buf
                .get_unchecked_mut(self.pos + 16)
                .as_mut_ptr()
                .write_volatile(0);
            self.buf
                .get_unchecked_mut(self.pos + 17)
                .as_mut_ptr()
                .write_volatile(0);
            self.buf
                .get_unchecked_mut(self.pos + 18)
                .as_mut_ptr()
                .write_volatile(0);
            self.buf
                .get_unchecked_mut(self.pos + 19)
                .as_mut_ptr()
                .write_volatile(0);
            self.buf
                .get_unchecked_mut(self.pos + 20)
                .as_mut_ptr()
                .write_volatile(0);
            self.buf
                .get_unchecked_mut(self.pos + 21)
                .as_mut_ptr()
                .write_volatile(0);
            self.buf
                .get_unchecked_mut(self.pos + 22)
                .as_mut_ptr()
                .write_volatile(0);
            self.buf
                .get_unchecked_mut(self.pos + 23)
                .as_mut_ptr()
                .write_volatile(0);
            self.buf
                .get_unchecked_mut(self.pos + 24)
                .as_mut_ptr()
                .write_volatile(0);
            self.buf
                .get_unchecked_mut(self.pos + 25)
                .as_mut_ptr()
                .write_volatile(0);
            self.buf
                .get_unchecked_mut(self.pos + 26)
                .as_mut_ptr()
                .write_volatile(0);
            self.buf
                .get_unchecked_mut(self.pos + 27)
                .as_mut_ptr()
                .write_volatile(0);
            self.buf
                .get_unchecked_mut(self.pos + 28)
                .as_mut_ptr()
                .write_volatile(0);
            self.buf
                .get_unchecked_mut(self.pos + 29)
                .as_mut_ptr()
                .write_volatile(0);
            self.buf
                .get_unchecked_mut(self.pos + 30)
                .as_mut_ptr()
                .write_volatile(0);
            self.buf
                .get_unchecked_mut(self.pos + 31)
                .as_mut_ptr()
                .write_volatile(0);
            self.buf
                .get_unchecked_mut(self.pos + 32)
                .as_mut_ptr()
                .write_volatile(0);
            self.buf
                .get_unchecked_mut(self.pos + 33)
                .as_mut_ptr()
                .write_volatile(0);
            self.buf
                .get_unchecked_mut(self.pos + 34)
                .as_mut_ptr()
                .write_volatile(0);
        }
        self.pos += 35;
    }

    /// Encodes an account.
    #[inline(always)]
    pub fn encode_account(&mut self, account_id: &AccountId, account_type: AccountType) {
        unsafe {
            let account_type: u8 = account_type.into();
            self.buf
                .get_unchecked_mut(self.pos)
                .as_mut_ptr()
                .write_volatile(0x80 + account_type);
            self.buf
                .get_unchecked_mut(self.pos + 1)
                .as_mut_ptr()
                .write_volatile(0x14);

            // avoid creating loops in the resulting wasm
            self.buf
                .get_unchecked_mut(self.pos + 2)
                .as_mut_ptr()
                .write_volatile(*account_id.get_unchecked(0));
            self.buf
                .get_unchecked_mut(self.pos + 3)
                .as_mut_ptr()
                .write_volatile(*account_id.get_unchecked(1));
            self.buf
                .get_unchecked_mut(self.pos + 4)
                .as_mut_ptr()
                .write_volatile(*account_id.get_unchecked(2));
            self.buf
                .get_unchecked_mut(self.pos + 5)
                .as_mut_ptr()
                .write_volatile(*account_id.get_unchecked(3));
            self.buf
                .get_unchecked_mut(self.pos + 6)
                .as_mut_ptr()
                .write_volatile(*account_id.get_unchecked(4));
            self.buf
                .get_unchecked_mut(self.pos + 7)
                .as_mut_ptr()
                .write_volatile(*account_id.get_unchecked(5));
            self.buf
                .get_unchecked_mut(self.pos + 8)
                .as_mut_ptr()
                .write_volatile(*account_id.get_unchecked(6));
            self.buf
                .get_unchecked_mut(self.pos + 9)
                .as_mut_ptr()
                .write_volatile(*account_id.get_unchecked(7));
            self.buf
                .get_unchecked_mut(self.pos + 10)
                .as_mut_ptr()
                .write_volatile(*account_id.get_unchecked(8));
            self.buf
                .get_unchecked_mut(self.pos + 11)
                .as_mut_ptr()
                .write_volatile(*account_id.get_unchecked(9));
            self.buf
                .get_unchecked_mut(self.pos + 12)
                .as_mut_ptr()
                .write_volatile(*account_id.get_unchecked(10));
            self.buf
                .get_unchecked_mut(self.pos + 13)
                .as_mut_ptr()
                .write_volatile(*account_id.get_unchecked(11));
            self.buf
                .get_unchecked_mut(self.pos + 14)
                .as_mut_ptr()
                .write_volatile(*account_id.get_unchecked(12));
            self.buf
                .get_unchecked_mut(self.pos + 15)
                .as_mut_ptr()
                .write_volatile(*account_id.get_unchecked(13));
            self.buf
                .get_unchecked_mut(self.pos + 16)
                .as_mut_ptr()
                .write_volatile(*account_id.get_unchecked(14));
            self.buf
                .get_unchecked_mut(self.pos + 17)
                .as_mut_ptr()
                .write_volatile(*account_id.get_unchecked(15));
            self.buf
                .get_unchecked_mut(self.pos + 18)
                .as_mut_ptr()
                .write_volatile(*account_id.get_unchecked(16));
            self.buf
                .get_unchecked_mut(self.pos + 19)
                .as_mut_ptr()
                .write_volatile(*account_id.get_unchecked(17));
            self.buf
                .get_unchecked_mut(self.pos + 20)
                .as_mut_ptr()
                .write_volatile(*account_id.get_unchecked(18));
            self.buf
                .get_unchecked_mut(self.pos + 21)
                .as_mut_ptr()
                .write_volatile(*account_id.get_unchecked(19));
        }
        self.pos += 22;
    }
}

impl<'a> XrpPaymentBuilder<'a> {
    /// Creates a new builder.
    #[inline(always)]
    pub fn new(drops: u64, to_address: &'a [u8; 20], dest_tag: u32, src_tag: u32) -> Self {
        Self {
            drops,
            to_address,
            dest_tag,
            src_tag,
        }
    }
}

impl<'a> TransactionBuilder<248> for XrpPaymentBuilder<'a> {
    const TXN_TYPE: TxnType = TxnType::Payment;

    #[inline(always)]
    fn build(self) -> Result<[u8; 248]> {
        let current_ledger_sequence = ledger_seq() as u32;
        let hook_account = match hook_account() {
            Err(e) => return Err(e),
            Ok(acc) => unsafe {
                acc.as_ptr().cast::<[u8; 20]>().read_volatile()
            },
        };
        let uninitialized_buffer: [MaybeUninit<u8>; 248] = MaybeUninit::uninit_array();
        let mut txn_buffer = TransactionBuffer {
            buf: unsafe { uninitialized_buffer.as_ptr().cast::<[MaybeUninit<u8>; 248]>().read_volatile() },
            pos: 0,
        };
        // transaction type
        txn_buffer.encode_txn_type(Self::TXN_TYPE);
        // flags
        txn_buffer.encode_u32(c::tfCANONICAL, FieldCode::Flags.into());
        // source tag
        txn_buffer.encode_u32(self.src_tag, FieldCode::SourceTag.into());
        // sequence
        txn_buffer.encode_u32(0, FieldCode::Sequence.into());
        // destination tag
        txn_buffer.encode_u32(self.dest_tag, FieldCode::DestinationTag.into());
        // first ledger sequence
        txn_buffer.encode_u32_with_field_id(
            current_ledger_sequence + 1,
            FieldCode::FirstLedgerSequence.into(),
        );
        // last ledger sequence
        txn_buffer.encode_u32_with_field_id(
            current_ledger_sequence + 5,
            FieldCode::LastLedgerSequence.into(),
        );
        // amount in drops
        txn_buffer.encode_drops(self.drops, AmountType::Amount);
        // fee in drops (fee will be calculated at the end, but we need to reserve space for it)
        let fee_pos = txn_buffer.pos;
        txn_buffer.encode_drops(0, AmountType::Fee);
        // signing public key, but it is always null
        txn_buffer.encode_signing_pubkey_as_null();
        // source account
        txn_buffer.encode_account(&hook_account, AccountType::Account);
        // // destination account
        txn_buffer.encode_account(self.to_address, AccountType::Destination);
        // transaction metadata
        let etxn_metadata = match etxn_details() {
            Err(e) => return Err(e),
            Ok(details) => unsafe { details.as_ptr().cast::<[u8; 105]>().read_volatile() },
        };
        let mut i = 0;
        while  {
            max_iter(106);
            i < 105
        } {
            unsafe {
                txn_buffer.buf.get_unchecked_mut(txn_buffer.pos + i)
                .as_mut_ptr()
                .write_volatile(*etxn_metadata.get_unchecked(i));
            }
            i += 1;
        }
        txn_buffer.pos += EMIT_DETAILS_SIZE;
        // let initialized_buffer = unsafe { MaybeUninit::array_assume_init(txn_buffer.buf) };
        let initialized_buffer = unsafe {
            // replacement of array_assume_init since it sometimes causes memcpy to be called
            // when the array is sufficiently large
             mem::transmute::<_, [u8; 248]>(
                txn_buffer.buf.as_ptr().cast::<[u8; 248]>().read_volatile()
            )
        };
        unsafe {
            // this way, memcpy is not called
            Ok(initialized_buffer.as_ptr().cast::<[u8; 248]>().read_volatile())
        }
    }
}

impl From<FieldCode> for u8 {
    #[inline(always)]
    fn from(field_code: FieldCode) -> Self {
        field_code as u8
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn can_encode_transaction_type() {
        use super::*;

        let txn_types = [
            TxnType::Payment,
            TxnType::EscrowCreate,
            TxnType::EscrowFinish,
            TxnType::AccountSet,
            TxnType::EscrowCancel,
            TxnType::RegularKeySet,
            TxnType::OfferCreate,
            TxnType::OfferCancel,
            TxnType::TicketCreate,
            TxnType::TicketCancel,
            TxnType::SignerListSet,
            TxnType::PaychanCreate,
            TxnType::PaychanFund,
            TxnType::PaychanClaim,
            TxnType::CheckCreate,
            TxnType::CheckCash,
            TxnType::CheckCancel,
            TxnType::DepositPreauth,
            TxnType::TrustSet,
            TxnType::AccountDelete,
            TxnType::HookSet,
            TxnType::Amendment,
            TxnType::Fee,
            TxnType::UnlModify,
        ];

        for txn_type in txn_types {
            let buf = [MaybeUninit::uninit(); 248];
            let mut txn_buffer = TransactionBuffer {
                buf,
                pos: 0,
            };
            txn_buffer.encode_txn_type(txn_type);
            let txn_type: u8 = txn_type.into(); 
            unsafe {
                assert_eq!(txn_buffer.buf[0].assume_init(), txn_type);
                assert_eq!(txn_buffer.buf[1].assume_init(), ((txn_type as u16 >> 8) & 0xFF) as u8);
                assert_eq!(txn_buffer.buf[2].assume_init(), (txn_type as u16 & 0xFF) as u8);
            }
        }
    }   
}