use core::mem::MaybeUninit;

use crate::api::*;
use crate::{c, hook_account, ledger_seq, AccountId, AccountType, AmountType, TxnType};

/// Builds a transaction to send XRP.
///
/// **Note that this only works with `cbak` function present in your hook code,
/// because the transaction buffer size is different when cbak does not exist,
/// but this case is not handled yet.**
///
/// Equivalent to `PREPARE_PAYMENT_SIMPLE` in `macro.h` in
/// official hooks API.
///
/// When successfully built, the transaction buffer will be 270 bytes long
/// that look like:
///
/// ```
/// 120000 // txn type (3 bytes)
/// 2280000000 // flags (5 bytes)
/// 2300000000 // source tag (5 bytes)
/// 2400000000 // sequence (5 bytes)
/// 2E00000000 // destination tag (5 bytes)
/// 201A0065D303 // first ledger sequence (6 bytes)
/// 201B0065D307 // last ledger sequence (6 bytes)
/// 6140000000000003E8 // amount to send (9 bytes)
/// 6840000000000000C7 // fee (9 bytes)
/// 7321000000000000000000000000000000000000000000000000000000000000000000 // pub key, signed as null (35 bytes)
/// 8114090A708604BC3BB4459F01E50AC0023FE682D2AD // source account (22 bytes)
/// 8314A8B7F78C0AE9FD42183EE45170D05F92F7F74239 // destination account (22 bytes)
/// ED202E000000013D00000000000000015B316CD7252B2F6A808CFBC98D9DD7C687316E850D7608647173A8793CD9553B2D5CB2D9188C36F2EEE397BCF9DAE609966A2F79C69275F57D7BD22DAB20ED037C765D2702C5E3E248D5DDBD1399D6AF79DB23FF37599BEA01AF2300985DA7BE52C0858A14090A708604BC3BB4459F01E50AC0023FE682D2ADE1 // txn details (138 bytes)
/// ```
///
/// # Example
///
/// ```
/// let xrp_payment_txn_builder = XrpPaymentBuilder::new(1000, &otxn_account, 0, 0);
/// let mut xrp_payment_txn_buffer = XrpPaymentBuilder::uninit_buffer();
/// match xrp_payment_txn_builder.build(&mut xrp_payment_txn_buffer) {
///     Ok(_) => {}
///     Err(err) => {
///         rollback(b"could not build xrp payment txn", err.into());
///     }
/// };
/// let txn_hash = match emit(&xrp_payment_txn_buffer) {
///     Ok(hash) => hash,
///     Err(err) => {
///         rollback(b"could not emit xrp payment txn", err.into());
///     }
/// };
/// ```
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
    /// Builds a specific transaction by directly modifying the uninitialized buffer provided
    /// as an argument.
    ///
    /// The reason that this function must take a mutable reference to an uninitialized buffer
    /// and cannot initialize its own buffer inside it to return it is because of Rust's
    /// restrictions on returning unsafe pointer to local variables to the outer scope
    /// and its compiler producing memcpy instructions on return a long buffer if we don't want to
    /// return a pointer to a local variable.
    ///
    /// For example, if we were to do this inside a function:
    /// ```
    /// let mut uninitialized_buffer: [MaybeUninit<u8>; 270] = MaybeUninit::uninit_array();
    ///
    /// ....
    ///
    /// return uninitialized_buffer.as_ptr() as *const u8
    /// ```
    ///
    /// This never works, because `uninitialized_buffer` is dropped at the end of the function,
    /// but the pointer to it is returned to the outer scope, which is undefined behavior.
    ///
    /// If we do this instead:
    ///
    /// ```
    /// let mut uninitialized_buffer: [MaybeUninit<u8>; 270] = MaybeUninit::uninit_array();
    ///
    /// ....
    ///
    /// return uninitialized_buffer
    /// ```
    ///
    /// This is completely fine as it just moves `uninitialized_buffer`, but it will
    /// somehow motivate the Rust compiler to create `memcpy` instruction, which is
    /// prohibited in wasm that is to be used as a hook where only two function exports
    /// are allowed: `hook` and `cbak`.
    ///
    /// # Example
    /// Therefore, we have no choice but to use this syntax:
    ///
    /// ```
    /// let xrp_payment_txn_builder = XrpPaymentBuilder::new(1000, &otxn_account, 0, 0);
    /// let mut buffer = XrpPaymentBuilder::uninit_buffer();
    /// match xrp_payment_txn_builder.build(&mut buffer) {
    ///     Ok(ptr) => ptr,
    ///     Err(err) => {
    ///         rollback(b"could not build xrp payment txn", err.into());
    ///     }
    /// };
    /// ```
    fn build(&self, uninitialized_buffer: &mut [MaybeUninit<u8>; TXN_LEN]) -> Result<()>;

    /// Utility method for creating an uninitialized buffer for a predefined length.
    /// Use this for creating an uninitialized transaction buffer to pass to `build`.
    ///
    /// # Example
    /// ```
    /// let xrp_payment_txn_builder = XrpPaymentBuilder::new(1000, &otxn_account, 0, 0);
    /// let mut buffer = XrpPaymentBuilder::uninit_buffer();
    /// match xrp_payment_txn_builder.build(&mut buffer) {
    ///     Ok(ptr) => ptr,
    ///     Err(err) => {
    ///         rollback(b"could not build xrp payment txn", err.into());
    ///     }
    /// };
    /// ```
    #[inline(always)]
    fn uninit_buffer() -> [MaybeUninit<u8>; TXN_LEN] {
        MaybeUninit::uninit_array()
    }
}

/// A generic buffer for building a transaction.
/// You can use this struct to build your custom transaction.
pub struct TransactionBuffer<'a, const TXN_LEN: usize> {
    buf: &'a mut [MaybeUninit<u8>; TXN_LEN],
    pos: usize,
}

// Ugly to specify the length of the transaction here instead
// of declaring it as an associated constant, but specifying
// constant has the return type in `build` method is unstable
// in Rust nightly right now. See `generic_const_exprs` feature.
impl<'a, const TXN_LEN: usize> TransactionBuffer<'a, TXN_LEN> {
    /// Encodes a transaction type.
    ///
    /// # Example
    /// ```
    /// let mut txn_buffer = ...
    ///
    /// txn_buffer.encode_txn_type(TxnType::Payment);
    /// ```
    #[inline(always)]
    pub fn encode_txn_type(&mut self, tt: TxnType) {
        unsafe {
            self.buf
                .get_unchecked_mut(self.pos)
                .as_mut_ptr()
                .write(FieldCode::TransactionType.into());
            self.buf
                .get_unchecked_mut(self.pos + 1)
                .as_mut_ptr()
                .write(((tt as u16 >> 8) & 0xFF) as u8);
            self.buf
                .get_unchecked_mut(self.pos + 2)
                .as_mut_ptr()
                .write((tt as u16 & 0xFF) as u8);
        }
        self.pos += 3;
    }

    /// Encodes a serialized field value for the first byte,
    /// and encodes the u32 data on the rest of the 4 bytes.
    ///
    /// Check [serialization format](https://xrpl.org/serialization.html) to see
    /// which field codes are available to be encoded using this method.
    ///
    /// # Example
    /// ```
    /// let mut txn_buffer = ...
    ///
    /// txn_buffer.encode_u32(c::tfCANONICAL, FieldCode::Flags.into());
    /// ```
    #[inline(always)]
    pub fn encode_u32(&mut self, data: u32, field: u8) {
        unsafe {
            self.buf
                .get_unchecked_mut(self.pos)
                .as_mut_ptr()
                .write(0x20 + (field & 0x0F));
            self.buf
                .get_unchecked_mut(self.pos + 1)
                .as_mut_ptr()
                .write(((data >> 24) & 0xFF) as u8);
            self.buf
                .get_unchecked_mut(self.pos + 2)
                .as_mut_ptr()
                .write(((data >> 16) & 0xFF) as u8);
            self.buf
                .get_unchecked_mut(self.pos + 3)
                .as_mut_ptr()
                .write(((data >> 8) & 0xFF) as u8);
            self.buf
                .get_unchecked_mut(self.pos + 4)
                .as_mut_ptr()
                .write((data & 0xFF) as u8);
        }
        self.pos += 5;
    }

    /// Encodes a u32 value with a field id. Note that
    /// the firsrt byte is always encoded as `0x20` and the second byte
    /// is always encoded as the field id. The rest of the 4 bytes are encoded
    /// with the u32 data.
    ///
    /// Check [serialization format](https://xrpl.org/serialization.html) to see
    /// which field codes are available to be encoded using this method.
    ///
    ///
    /// # Example
    /// ```
    /// let mut txn_buffer = ...
    ///
    /// txn_buffer.encode_u32_with_field_id(1000, FieldCode::Sequence.into());
    /// ```
    #[inline(always)]
    pub fn encode_u32_with_field_id(&mut self, data: u32, field: u8) {
        unsafe {
            self.buf
                .get_unchecked_mut(self.pos)
                .as_mut_ptr()
                .write(0x20);
            self.buf
                .get_unchecked_mut(self.pos + 1)
                .as_mut_ptr()
                .write(field);
            self.buf
                .get_unchecked_mut(self.pos + 2)
                .as_mut_ptr()
                .write(((data >> 24) & 0xFF) as u8);
            self.buf
                .get_unchecked_mut(self.pos + 3)
                .as_mut_ptr()
                .write(((data >> 16) & 0xFF) as u8);
            self.buf
                .get_unchecked_mut(self.pos + 4)
                .as_mut_ptr()
                .write(((data >> 8) & 0xFF) as u8);
            self.buf
                .get_unchecked_mut(self.pos + 5)
                .as_mut_ptr()
                .write((data & 0xFF) as u8);
        }
        self.pos += 6;
    }

    /// Encodes amount in drops.
    ///
    /// # Example
    /// ```
    /// let mut txn_buffer = ...
    ///
    /// txn_buffer.encode_drops(12, AmountType::Fee);
    /// ```
    #[inline(always)]
    pub fn encode_drops(&mut self, drops: u64, amount_type: AmountType) {
        self.encode_drops_at(self.pos, drops, amount_type);
    }

    /// Encodes amount in drops at a specific position.
    ///
    /// # Example
    /// ```
    /// let mut txn_buffer = ...
    ///
    /// let fee = etxn_fee_base(&txn_buffer);
    ///
    /// txn_buffer.encode_drops_at(45, fee, AmountType::Fee);
    /// ```
    #[inline(always)]
    pub fn encode_drops_at(&mut self, pos: usize, drops: u64, amount_type: AmountType) {
        let amount_type: u8 = amount_type.into();
        unsafe {
            self.buf
                .get_unchecked_mut(pos)
                .as_mut_ptr()
                .write(0x60 + (amount_type & 0x0F));
            self.buf
                .get_unchecked_mut(pos + 1)
                .as_mut_ptr()
                .write((0b01000000 + ((drops >> 56) & 0b00111111)) as u8);
            self.buf
                .get_unchecked_mut(pos + 2)
                .as_mut_ptr()
                .write(((drops >> 48) & 0xFF) as u8);
            self.buf
                .get_unchecked_mut(pos + 3)
                .as_mut_ptr()
                .write(((drops >> 40) & 0xFF) as u8);
            self.buf
                .get_unchecked_mut(pos + 4)
                .as_mut_ptr()
                .write(((drops >> 32) & 0xFF) as u8);
            self.buf
                .get_unchecked_mut(pos + 5)
                .as_mut_ptr()
                .write(((drops >> 24) & 0xFF) as u8);
            self.buf
                .get_unchecked_mut(pos + 6)
                .as_mut_ptr()
                .write(((drops >> 16) & 0xFF) as u8);
            self.buf
                .get_unchecked_mut(pos + 7)
                .as_mut_ptr()
                .write(((drops >> 8) & 0xFF) as u8);
            self.buf
                .get_unchecked_mut(pos + 8)
                .as_mut_ptr()
                .write((drops & 0xFF) as u8);
        }
        self.pos += 9;
    }

    /// Encodes an amount in drops at a specific position of the buffer.
    ///
    /// # Safety
    /// `pos` must be a valid position in the buffer where the fee should be encoded at.
    /// and the buffer must be initialized up to `pos + 9`.
    ///
    /// only call this function when you get the fee from [etxn_fee_base_from_ptr] or [etxn_fee_base]
    /// and want to encode the fee.
    ///
    /// # Example
    /// ```
    /// let mut txn_buffer = ...
    ///
    /// ...
    ///
    /// let fee = etxn_fee_base(&txn_buffer);
    ///
    /// txn_buffer.encode_drops_at_buf_ptr(unsafe { txn_buffer.as_mut_ptr() }, 45, fee, AmountType::Fee);
    /// ```
    #[inline(always)]
    pub unsafe fn encode_drops_at_buf_ptr(
        uninitialized_buf: *mut MaybeUninit<u8>,
        pos: usize,
        drops: u64,
        amount_type: AmountType,
    ) {
        let amount_type: u8 = amount_type.into();
        let mut pos_0 = MaybeUninit::uninit();
        let mut pos_1 = MaybeUninit::uninit();
        let mut pos_2 = MaybeUninit::uninit();
        let mut pos_3 = MaybeUninit::uninit();
        let mut pos_4 = MaybeUninit::uninit();
        let mut pos_5 = MaybeUninit::uninit();
        let mut pos_6 = MaybeUninit::uninit();
        let mut pos_7 = MaybeUninit::uninit();
        let mut pos_8 = MaybeUninit::uninit();

        pos_0.write(0x60 + (amount_type & 0x0F));
        pos_1.write((0b01000000 + ((drops >> 56) & 0b00111111)) as u8);
        pos_2.write(((drops >> 48) & 0xFF) as u8);
        pos_3.write(((drops >> 40) & 0xFF) as u8);
        pos_4.write(((drops >> 32) & 0xFF) as u8);
        pos_5.write(((drops >> 24) & 0xFF) as u8);
        pos_6.write(((drops >> 16) & 0xFF) as u8);
        pos_7.write(((drops >> 8) & 0xFF) as u8);
        pos_8.write((drops & 0xFF) as u8);
        unsafe {
            uninitialized_buf.add(pos).write(pos_0);
            uninitialized_buf.add(pos + 1).write(pos_1);
            uninitialized_buf.add(pos + 2).write(pos_2);
            uninitialized_buf.add(pos + 3).write(pos_3);
            uninitialized_buf.add(pos + 4).write(pos_4);
            uninitialized_buf.add(pos + 5).write(pos_5);
            uninitialized_buf.add(pos + 6).write(pos_6);
            uninitialized_buf.add(pos + 7).write(pos_7);
            uninitialized_buf.add(pos + 8).write(pos_8);
        }
    }

    /// Encodes a signing public key as null. For transactions
    /// emitted from hooks, the signing public key is always null.
    ///
    /// # Example
    /// ```
    /// let mut txn_buffer = ...
    ///
    /// txn_buffer.encode_signing_pubkey_as_null();
    /// ```
    #[inline(always)]
    pub fn encode_signing_pubkey_as_null(&mut self) {
        // leave self.buf[self.pos + 2..self.pos + 35] as 0 because they
        // are already initialized to 0 and meant to be like that to
        // represent null
        unsafe {
            self.buf
                .get_unchecked_mut(self.pos)
                .as_mut_ptr()
                .write(0x73);
            self.buf
                .get_unchecked_mut(self.pos + 1)
                .as_mut_ptr()
                .write(0x21);

            // avoid creating loops in the resulting wasm
            let u64_ptr = self.buf.get_unchecked_mut(self.pos + 2).as_mut_ptr() as *mut u64;
            u64_ptr.write(0);
            u64_ptr.offset(1).write(0);
            u64_ptr.offset(2).write(0);
            u64_ptr.offset(3).write(0); // total 32 bytes of 0
        }
        self.pos += 35;
    }

    /// Encodes an account.
    ///
    /// # Example
    /// ```
    /// let mut txn_buffer = ...
    ///
    /// txn_buffer.encode_account(&otxn_account, AccountType::Account);
    /// ```
    #[inline(always)]
    pub fn encode_account(&mut self, account_id: &AccountId, account_type: AccountType) {
        unsafe {
            let account_type: u8 = account_type.into();
            self.buf
                .get_unchecked_mut(self.pos)
                .as_mut_ptr()
                .write(0x80 + account_type);
            self.buf
                .get_unchecked_mut(self.pos + 1)
                .as_mut_ptr()
                .write(0x14);

            let u64_account_id_ptr = account_id.as_ptr() as *mut u64;
            let u64_buf_ptr = self.buf.get_unchecked_mut(self.pos + 2).as_ptr() as *mut u64;

            // 16 bytes written
            u64_buf_ptr.write(u64_account_id_ptr.read());
            u64_buf_ptr
                .offset(1)
                .write(u64_account_id_ptr.offset(1).read());

            let u32_account_id_ptr = account_id.as_ptr().offset(16) as *mut u32;
            let u32_buf_ptr = self.buf.get_unchecked_mut(self.pos + 18).as_ptr() as *mut u32;

            // 4 bytes written
            u32_buf_ptr.write(u32_account_id_ptr.read());
        }
        self.pos += 22;
    }
}

impl<'a> XrpPaymentBuilder<'a> {
    /// Creates a new builder for XRP payment.
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

impl<'a> TransactionBuilder<270> for XrpPaymentBuilder<'a> {
    const TXN_TYPE: TxnType = TxnType::Payment;

    #[inline(always)]
    fn build(
        &self,
        uninitialized_buffer: &mut [MaybeUninit<u8>; XrpPaymentBuilder::TXN_LEN],
    ) -> Result<()> {
        let current_ledger_sequence = ledger_seq() as u32;
        let hook_account = match hook_account() {
            Err(e) => return Err(e),
            Ok(acc) => acc,
        };
        let mut txn_buffer = TransactionBuffer {
            buf: uninitialized_buffer,
            pos: 0,
        };

        // transaction type
        txn_buffer.encode_txn_type(Self::TXN_TYPE); // pos = 3

        // flags
        txn_buffer.encode_u32(c::tfCANONICAL, FieldCode::Flags.into()); // pos = 8

        // source tag
        txn_buffer.encode_u32(self.src_tag, FieldCode::SourceTag.into()); // pos = 13

        // sequence
        txn_buffer.encode_u32(0, FieldCode::Sequence.into()); // pos = 18

        // destination tag
        txn_buffer.encode_u32(self.dest_tag, FieldCode::DestinationTag.into()); // pos = 23

        // first ledger sequence
        txn_buffer.encode_u32_with_field_id(
            current_ledger_sequence + 1,
            FieldCode::FirstLedgerSequence.into(),
        ); // pos = 29

        // last ledger sequence
        txn_buffer.encode_u32_with_field_id(
            current_ledger_sequence + 5,
            FieldCode::LastLedgerSequence.into(),
        ); // pos = 35

        // amount in drops
        txn_buffer.encode_drops(self.drops, AmountType::Amount); // pos = 44

        // fee in drops (fee will be calculated at the end, but we need to reserve space for it)
        let fee_pos = txn_buffer.pos;
        txn_buffer.encode_drops(0, AmountType::Fee); // pos = 53

        // signing public key, but it is always null
        txn_buffer.encode_signing_pubkey_as_null(); // pos = 88

        // source account
        txn_buffer.encode_account(&hook_account, AccountType::Account); // pos = 110

        // destination account
        txn_buffer.encode_account(self.to_address, AccountType::Destination); // pos = 132

        let buf_mut_ptr = txn_buffer.buf.as_mut_ptr();
        // transaction metadata
        let insert_etxn_details_from_ptr_result: Result<u64> =
            insert_etxn_details_from_ptr(unsafe { buf_mut_ptr.add(txn_buffer.pos) as u32 }, 138);
        match insert_etxn_details_from_ptr_result {
            Err(e) => return Err(e),
            Ok(_) => {}
        }
        txn_buffer.pos += 138; // pos = 270

        // encode fee because we have the full transaction now
        let fee = match etxn_fee_base_from_ptr(buf_mut_ptr, XrpPaymentBuilder::TXN_LEN) {
            Err(e) => return Err(e),
            Ok(fee) => fee,
        };

        unsafe {
            TransactionBuffer::<{ XrpPaymentBuilder::TXN_LEN }>::encode_drops_at_buf_ptr(
                buf_mut_ptr,
                fee_pos,
                fee,
                AmountType::Fee,
            )
        };

        Ok(())
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
    use core::mem::MaybeUninit;

    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::{AccountType, AmountType, TransactionBuffer, ACC_ID_LEN};

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
            let mut uninitialized_buffer: [MaybeUninit<u8>; 270] = MaybeUninit::uninit_array();
            for i in 0..270 {
                unsafe {
                    uninitialized_buffer
                        .get_unchecked_mut(i)
                        .as_mut_ptr()
                        .write(0);
                }
            }
            let mut txn_buffer = TransactionBuffer {
                buf: &mut uninitialized_buffer,
                pos: 0,
            };
            txn_buffer.encode_txn_type(txn_type);
            let txn_type: u8 = txn_type.into();
            unsafe {
                assert_eq!(txn_buffer.buf[0].assume_init(), 0x12);
                assert_eq!(
                    txn_buffer.buf[1].assume_init(),
                    ((txn_type as u16 >> 8) & 0xFF) as u8
                );
                assert_eq!(
                    txn_buffer.buf[2].assume_init(),
                    (txn_type as u16 & 0xFF) as u8
                );
            }
            assert_eq!(txn_buffer.pos, 3);
        }
    }

    #[wasm_bindgen_test]
    fn can_encode_drops_at_buf_ptr() {
        let mut uninitialized_buffer: [MaybeUninit<u8>; 270] = MaybeUninit::uninit_array();
        // avoid undefined behavior when calling array_assume_init
        for i in 0..270 {
            unsafe {
                uninitialized_buffer
                    .get_unchecked_mut(i)
                    .as_mut_ptr()
                    .write(0);
            }
        }
        unsafe {
            TransactionBuffer::<270>::encode_drops_at_buf_ptr(
                uninitialized_buffer.as_mut_ptr(),
                44,
                12_u64,
                AmountType::Fee,
            );
        }
        assert_eq!(
            unsafe { MaybeUninit::array_assume_init(uninitialized_buffer) },
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 104, 64, 0, 0, 0, 0, 0, 0, 12, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
    }

    #[wasm_bindgen_test]
    fn can_encode_account() {
        let account: [u8; ACC_ID_LEN] = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ];
        let mut uninitialized_buffer: [MaybeUninit<u8>; 270] = MaybeUninit::uninit_array();
        for i in 0..270 {
            unsafe {
                uninitialized_buffer
                    .get_unchecked_mut(i)
                    .as_mut_ptr()
                    .write(0);
            }
        }
        let mut txn_buffer = TransactionBuffer {
            buf: &mut uninitialized_buffer,
            pos: 15,
        };
        txn_buffer.encode_account(&account, AccountType::Account);

        assert_eq!(txn_buffer.pos, 37);
        assert_eq!(
            unsafe { MaybeUninit::array_assume_init(uninitialized_buffer) },
            [
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0x80 + AccountType::Account as u8,
                0x14,
                1,
                2,
                3,
                4,
                5,
                6,
                7,
                8,
                9,
                10,
                11,
                12,
                13,
                14,
                15,
                16,
                17,
                18,
                19,
                20,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0
            ]
        )
    }

    #[wasm_bindgen_test]
    fn can_encode_signing_pupkey_as_null() {
        let mut uninitialized_buffer: [MaybeUninit<u8>; 270] = MaybeUninit::uninit_array();
        for i in 0..270 {
            unsafe {
                uninitialized_buffer
                    .get_unchecked_mut(i)
                    .as_mut_ptr()
                    .write(0);
            }
        }
        let mut txn_buffer = TransactionBuffer {
            buf: &mut uninitialized_buffer,
            pos: 30,
        };
        txn_buffer.encode_signing_pubkey_as_null();

        assert_eq!(txn_buffer.pos, 65);
        assert_eq!(
            unsafe { MaybeUninit::array_assume_init(uninitialized_buffer) },
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0x73, 0x21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        )
    }
}
