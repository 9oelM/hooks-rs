// allow dead code
#![allow(dead_code)]
// allow dead imports
#![allow(unused_imports)]
// allow unused var
#![allow(unused_variables)]
use core::mem::MaybeUninit;

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
            self.buf.get_unchecked_mut(self.pos).write(FieldCode::TransactionType.into());
            self.buf.get_unchecked_mut(self.pos + 1).write(
                ((tt as u16 >> 8) & 0xFF) as u8
            );
            self.buf.get_unchecked_mut(self.pos + 2).write(
                (tt as u16 & 0xFF) as u8
            );
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
            self.buf.get_unchecked_mut(self.pos).write(0x20 + (field & 0x0F));
            self.buf.get_unchecked_mut(self.pos + 1).write(
                ((data >> 24) & 0xFF) as u8
            );
            self.buf.get_unchecked_mut(self.pos + 2).write(
                ((data >> 16) & 0xFF) as u8
            );
            self.buf.get_unchecked_mut(self.pos + 3).write(
                ((data >> 8) & 0xFF) as u8
            );
            self.buf.get_unchecked_mut(self.pos + 4).write(
                (data & 0xFF) as u8
            );
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
            self.buf.get_unchecked_mut(self.pos).write(0x20);
            self.buf.get_unchecked_mut(self.pos + 1).write(field);
            self.buf.get_unchecked_mut(self.pos + 2).write(
                ((data >> 24) & 0xFF) as u8
            );
            self.buf.get_unchecked_mut(self.pos + 3).write(
                ((data >> 16) & 0xFF) as u8
            );
            self.buf.get_unchecked_mut(self.pos + 4).write(
                ((data >> 8) & 0xFF) as u8
            );
            self.buf.get_unchecked_mut(self.pos + 5).write(
                (data & 0xFF) as u8
            );
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
            self.buf.get_unchecked_mut(pos).write(0x60 + (amount_type & 0x0F));
            self.buf.get_unchecked_mut(pos + 1).write(
                (0b01000000 + ((drops >> 56) & 0b00111111)) as u8
            );
            self.buf.get_unchecked_mut(pos + 2).write(
                ((drops >> 48) & 0xFF) as u8
            );
            self.buf.get_unchecked_mut(pos + 3).write(
                ((drops >> 40) & 0xFF) as u8
            );
            self.buf.get_unchecked_mut(pos + 4).write(
                ((drops >> 32) & 0xFF) as u8
            );
            self.buf.get_unchecked_mut(pos + 5).write(
                ((drops >> 24) & 0xFF) as u8
            );
            self.buf.get_unchecked_mut(pos + 6).write(
                ((drops >> 16) & 0xFF) as u8
            );
            self.buf.get_unchecked_mut(pos + 7).write(
                ((drops >> 8) & 0xFF) as u8
            );
            self.buf.get_unchecked_mut(pos + 8).write(
                (drops & 0xFF) as u8
            );
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
            self.buf.get_unchecked_mut(self.pos).write(0x73);
            self.buf.get_unchecked_mut(self.pos + 1).write(0x21);

            // avoid creating loops in the resulting wasm
            self.buf.get_unchecked_mut(self.pos + 2).write(0);
            self.buf.get_unchecked_mut(self.pos + 3).write(0);
            self.buf.get_unchecked_mut(self.pos + 4).write(0);
            self.buf.get_unchecked_mut(self.pos + 5).write(0);
            self.buf.get_unchecked_mut(self.pos + 6).write(0);
            self.buf.get_unchecked_mut(self.pos + 7).write(0);
            self.buf.get_unchecked_mut(self.pos + 8).write(0);
            self.buf.get_unchecked_mut(self.pos + 9).write(0);
            self.buf.get_unchecked_mut(self.pos + 10).write(0);
            self.buf.get_unchecked_mut(self.pos + 11).write(0);
            self.buf.get_unchecked_mut(self.pos + 12).write(0);
            self.buf.get_unchecked_mut(self.pos + 13).write(0);
            self.buf.get_unchecked_mut(self.pos + 14).write(0);
            self.buf.get_unchecked_mut(self.pos + 15).write(0);
            self.buf.get_unchecked_mut(self.pos + 16).write(0);
            self.buf.get_unchecked_mut(self.pos + 17).write(0);
            self.buf.get_unchecked_mut(self.pos + 18).write(0);
            self.buf.get_unchecked_mut(self.pos + 19).write(0);
            self.buf.get_unchecked_mut(self.pos + 20).write(0);
            self.buf.get_unchecked_mut(self.pos + 21).write(0);
            self.buf.get_unchecked_mut(self.pos + 22).write(0);
            self.buf.get_unchecked_mut(self.pos + 23).write(0);
            self.buf.get_unchecked_mut(self.pos + 24).write(0);
            self.buf.get_unchecked_mut(self.pos + 25).write(0);
            self.buf.get_unchecked_mut(self.pos + 26).write(0);
            self.buf.get_unchecked_mut(self.pos + 27).write(0);
            self.buf.get_unchecked_mut(self.pos + 28).write(0);
            self.buf.get_unchecked_mut(self.pos + 29).write(0);
            self.buf.get_unchecked_mut(self.pos + 30).write(0);
            self.buf.get_unchecked_mut(self.pos + 31).write(0);
            self.buf.get_unchecked_mut(self.pos + 32).write(0);
            self.buf.get_unchecked_mut(self.pos + 33).write(0);
            self.buf.get_unchecked_mut(self.pos + 34).write(0);
        }
        self.pos += 35;
    }

    /// Encodes an account.
    #[inline(always)]
    pub fn encode_account(&mut self, account_id: &AccountId, account_type: AccountType) {
        unsafe {
            let account_type: u8 = account_type.into();
            self.buf.get_unchecked_mut(self.pos).write(0x80 + account_type);
            self.buf.get_unchecked_mut(self.pos + 1).write(0x14);

            // avoid creating loops in the resulting wasm
            self.buf.get_unchecked_mut(self.pos + 2).write(*account_id.get_unchecked(0));
            self.buf.get_unchecked_mut(self.pos + 3).write(*account_id.get_unchecked(1));
            self.buf.get_unchecked_mut(self.pos + 4).write(*account_id.get_unchecked(2));
            self.buf.get_unchecked_mut(self.pos + 5).write(*account_id.get_unchecked(3));
            self.buf.get_unchecked_mut(self.pos + 6).write(*account_id.get_unchecked(4));
            self.buf.get_unchecked_mut(self.pos + 7).write(*account_id.get_unchecked(5));
            self.buf.get_unchecked_mut(self.pos + 8).write(*account_id.get_unchecked(6));
            self.buf.get_unchecked_mut(self.pos + 9).write(*account_id.get_unchecked(7));
            self.buf.get_unchecked_mut(self.pos + 10).write(*account_id.get_unchecked(8));
            self.buf.get_unchecked_mut(self.pos + 11).write(*account_id.get_unchecked(9));
            self.buf.get_unchecked_mut(self.pos + 12).write(*account_id.get_unchecked(10));
            self.buf.get_unchecked_mut(self.pos + 13).write(*account_id.get_unchecked(11));
            self.buf.get_unchecked_mut(self.pos + 14).write(*account_id.get_unchecked(12));
            self.buf.get_unchecked_mut(self.pos + 15).write(*account_id.get_unchecked(13));
            self.buf.get_unchecked_mut(self.pos + 16).write(*account_id.get_unchecked(14));
            self.buf.get_unchecked_mut(self.pos + 17).write(*account_id.get_unchecked(15));
            self.buf.get_unchecked_mut(self.pos + 18).write(*account_id.get_unchecked(16));
            self.buf.get_unchecked_mut(self.pos + 19).write(*account_id.get_unchecked(17));
            self.buf.get_unchecked_mut(self.pos + 20).write(*account_id.get_unchecked(18));
            self.buf.get_unchecked_mut(self.pos + 21).write(*account_id.get_unchecked(19));
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
            Ok(acc) => acc,
        };
        let uninitialized_buffer: [MaybeUninit<u8>; 248] = MaybeUninit::uninit_array();
        let mut txn_buffer = TransactionBuffer {
            buf: uninitialized_buffer,
            pos: 0,
        };
        // // transaction type
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
        // txn_buffer.encode_account(self.to_address, AccountType::Destination);
        // // transaction metadata
        // let etxn_metadata = match etxn_details() {
        //     Err(e) => return Err(e),
        //     Ok(details) => details,
        // };
        // unsafe {
        //     let new_slice = core::slice::from_raw_parts_mut(
        //         etxn_metadata.as_mut_ptr(), EMIT_DETAILS_SIZE);

        //     core::ptr::copy_nonoverlapping(txn_buffer.buf.as_mut_ptr().add(txn_buffer.pos), new_slice.as_mut_ptr(), EMIT_DETAILS_SIZE);
        // }
        // match insert_etxn_details(txn_buffer.buf[txn_buffer.pos..txn_buffer.pos + EMIT_DETAILS_SIZE].as_mut_ptr() as u32) {
        //     Err(e) => return Err(e),
        //     Ok(_) => (),
        // };
        // let mut i = 0;
        // while  {
        //     max_iter(106);
        //     i < 105
        // } {
        //     unsafe {
        //         txn_buffer.buf.get_unchecked_mut(txn_buffer.pos + i).write(*etxn_metadata.get_unchecked(
        //             i
        //         ));
        //     }
        //     i += 1;
        // }
        txn_buffer.pos += EMIT_DETAILS_SIZE;
        let initialized_buffer = unsafe {
            MaybeUninit::array_assume_init(txn_buffer.buf)
        };
        // let fee = etxn_fee_base(initialized_buffer.as_ref());
        // txn_buffer.encode_drops_at(fee_pos, fee as u64, AmountType::Fee);
        Ok(initialized_buffer)
    }
}

impl From<FieldCode> for u8 {
    #[inline(always)]
    fn from(field_code: FieldCode) -> Self {
        field_code as u8
    }
}
