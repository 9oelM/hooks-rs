use core::ops::Range;

use crate::api::*;

struct IntArray<T, const N: usize>
where
    T: PartialEq,
{
    data: [T; N],
}

/// Tests two buffers for equality
///
/// Pay attention to the GUARD_ID parameter.
/// This should be unique on every call, through the entire hook code.
/// Otherwise you will encounter guard violation during the execution of your hook.
#[inline(always)]
pub fn is_buffer_equal<T: PartialEq>(buf_1: &[T], buf_2: &[T]) -> bool {
    let buf1_len = buf_1.len();

    if buf1_len != buf_2.len() {
        return false;
    };

    // guarded loop
    let mut i = 0;
    while {
        max_iter(buf1_len as u32 + 1);
        i < buf1_len
    } {
        if buf_1[i] != buf_2[i] {
            return false;
        }
        i += 1;
    }

    true
}

/// Zeroize a buffer
///
/// Pay attention to the GUARD_ID parameter.
/// This should be unique on every call, through the entire hook code.
/// Otherwise you will encounter guard violation during the execution of your hook.
#[inline(always)]
pub fn buffer_zeroize<const GUARD_ID: u32>(buf: &mut [u8]) {
    let buf_len = buf.len();
    // guarded loop
    let mut i = 0;
    while {
        max_iter(buf_len as u32 + 1);
        i < buf_len
    } {
        buf[0] = 0;
        i += 1;
    }
}

/// Convert amount to drops
#[inline(always)]
pub const fn amount_to_drops(amount_buf: &Amount) -> Result<u64> {
    if (amount_buf[0] >> 7) == 1 {
        return Err(Error::InternalError);
    }

    Ok((((amount_buf[0] as u64) & 0xb00111111) << 56)
        + ((amount_buf[1] as u64) << 48)
        + ((amount_buf[2] as u64) << 40)
        + ((amount_buf[3] as u64) << 32)
        + ((amount_buf[4] as u64) << 24)
        + ((amount_buf[5] as u64) << 16)
        + ((amount_buf[6] as u64) << 8)
        + (amount_buf[7] as u64))
}

/// Prepares payment for emitting
#[inline(always)]
pub fn prepare_payment_simple(
    buf_out: &mut TxnPaymentSimple,
    drops_amount: u64,
    drops_fee: u64,
    to_address: &AccountId,
    dest_tag: u32,
    src_tag: u32,
) -> Result<()> {
    const TT_RANGE: Range<usize> = Range { start: 0, end: 3 };
    const FLAGS_RANGE: Range<usize> = Range { start: 3, end: 8 };
    const TAG_SRC_RANGE: Range<usize> = Range { start: 8, end: 13 };
    const SEQUENCE_RANGE: Range<usize> = Range { start: 13, end: 18 };
    const TAG_DST_RANGE: Range<usize> = Range { start: 18, end: 23 };
    const FLS_RANGE: Range<usize> = Range { start: 23, end: 29 };
    const LLS_RANGE: Range<usize> = Range { start: 29, end: 35 };
    const DROPS_RANGE: Range<usize> = Range { start: 35, end: 44 };
    const DROPS_FEE_RANGE: Range<usize> = Range { start: 44, end: 53 };
    const SIGNING_PUBKEY_RANGE: Range<usize> = Range { start: 53, end: 88 };
    const ACCOUNT_SRC_RANGE: Range<usize> = Range {
        start: 88,
        end: 110,
    };
    const ACCOUNT_DST_RANGE: Range<usize> = Range {
        start: 110,
        end: 132,
    };
    const ETXN_DETAILS_RANGE: Range<usize> = Range {
        start: 132,
        end: 237,
    };

    let acc = match hook_account() {
        Err(e) => return Err(e),
        Ok(acc) => acc,
    };

    let cls = ledger_seq() as u32;

    encode_tt(&mut buf_out[TT_RANGE], TxnType::Payment);
    encode_flags(&mut buf_out[FLAGS_RANGE], TF_CANONICAL);
    encode_tag_src(&mut buf_out[TAG_SRC_RANGE], src_tag);
    encode_sequence(&mut buf_out[SEQUENCE_RANGE], 0);
    encode_tag_dst(&mut buf_out[TAG_DST_RANGE], dest_tag);
    encode_fls(&mut buf_out[FLS_RANGE], cls + 1);
    encode_lls(&mut buf_out[LLS_RANGE], cls + 5);
    encode_drops_amount(&mut buf_out[DROPS_RANGE], drops_amount);
    encode_drops_fee(&mut buf_out[DROPS_FEE_RANGE], drops_fee);
    encode_signing_pubkey_null(&mut buf_out[SIGNING_PUBKEY_RANGE]);
    encode_account_src(&mut buf_out[ACCOUNT_SRC_RANGE], &acc);
    encode_account_dst(&mut buf_out[ACCOUNT_DST_RANGE], to_address);
    let details = match etxn_details() {
        Err(e) => return Err(e),
        Ok(details) => details,
    };
    buf_out[ETXN_DETAILS_RANGE].clone_from_slice(&details);

    Ok(())
}

#[inline(always)]
fn encode_tt(buf_out: &mut [u8], tt: TxnType) {
    buf_out[0] = 0x12;
    buf_out[1] = ((tt as u16 >> 8) & 0xFF) as u8;
    buf_out[2] = (tt as u16 & 0xFF) as u8;
}

#[inline(always)]
fn encode_flags(buf_out: &mut [u8], flags: u32) {
    encode_u32_common(buf_out, flags, 0x2)
}

#[inline(always)]
fn encode_tag_src(buf_out: &mut [u8], tag: u32) {
    encode_u32_common(buf_out, tag, 0x3)
}

#[inline(always)]
fn encode_sequence(buf_out: &mut [u8], sequence: u32) {
    encode_u32_common(buf_out, sequence, 0x4)
}

#[inline(always)]
fn encode_tag_dst(buf_out: &mut [u8], tag: u32) {
    encode_u32_common(buf_out, tag, 0xE)
}

#[inline(always)]
fn encode_fls(buf_out: &mut [u8], fls: u32) {
    encode_u32_uncommon(buf_out, fls, 0x1A)
}

#[inline(always)]
fn encode_lls(buf_out: &mut [u8], lls: u32) {
    encode_u32_uncommon(buf_out, lls, 0x1B)
}

#[inline(always)]
fn encode_drops_amount(buf_out: &mut [u8], drops: u64) {
    encode_drops(buf_out, drops, AmountType::Amount)
}

#[inline(always)]
fn encode_drops_fee(buf_out: &mut [u8], drops: u64) {
    encode_drops(buf_out, drops, AmountType::Fee)
}

#[inline(always)]
fn encode_account_src(buf_out: &mut [u8], account_id: &Buffer<ACC_ID_LEN>) {
    encode_account(buf_out, account_id, AccountType::Account)
}

#[inline(always)]
fn encode_account_dst(buf_out: &mut [u8], account_id: &Buffer<ACC_ID_LEN>) {
    encode_account(buf_out, account_id, AccountType::Destination)
}

#[inline(always)]
fn encode_u32_common(buf_out: &mut [u8], i: u32, field: u8) {
    buf_out[0] = 0x20 + (field & 0x0F);
    buf_out[1] = ((i >> 24) & 0xFF) as u8;
    buf_out[2] = ((i >> 16) & 0xFF) as u8;
    buf_out[3] = ((i >> 8) & 0xFF) as u8;
    buf_out[4] = (i & 0xFF) as u8;
}

#[inline(always)]
fn encode_u32_uncommon(buf_out: &mut [u8], i: u32, field: u8) {
    buf_out[0] = 0x20;
    buf_out[1] = field;
    buf_out[2] = ((i >> 24) & 0xFF) as u8;
    buf_out[3] = ((i >> 16) & 0xFF) as u8;
    buf_out[4] = ((i >> 8) & 0xFF) as u8;
    buf_out[5] = (i & 0xFF) as u8;
}

#[inline(always)]
fn encode_drops(buf_out: &mut [u8], drops: u64, amount_type: AmountType) {
    buf_out[0] = 0x60 + (amount_type as u8 & 0x0F);
    buf_out[1] = (0b01000000 + ((drops >> 56) & 0b00111111)) as u8;
    buf_out[2] = ((drops >> 48) & 0xFF) as u8;
    buf_out[3] = ((drops >> 40) & 0xFF) as u8;
    buf_out[4] = ((drops >> 32) & 0xFF) as u8;
    buf_out[5] = ((drops >> 24) & 0xFF) as u8;
    buf_out[6] = ((drops >> 16) & 0xFF) as u8;
    buf_out[7] = ((drops >> 8) & 0xFF) as u8;
    buf_out[8] = (drops & 0xFF) as u8;
}

#[inline(always)]
fn encode_signing_pubkey_null(buf_out: &mut [u8]) {
    buf_out[0] = 0x73;
    buf_out[1] = 0x21;
    buf_out[2..35].clone_from_slice(&[0; 33]);
}

#[inline(always)]
fn encode_account(buf_out: &mut [u8], account_id: &AccountId, account_type: AccountType) {
    buf_out[0] = 0x80 + account_type as u8;
    buf_out[1] = 0x14;
    buf_out[2..22].clone_from_slice(&account_id[..]);
}

impl<T: PartialEq, const N: usize> AsRef<[T]> for IntArray<T, N> {
    fn as_ref(&self) -> &[T] {
        self.data.as_ref()
    }
}

impl<T, const N: usize> PartialEq for IntArray<T, N>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        is_buffer_equal::<T>(self.as_ref(), other.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::c;
    use wasm_bindgen_test::*;

    const ACCOUNT_ID: AccountId = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ];

    #[wasm_bindgen_test]
    fn enc_account() {
        let mut encoded: [u8; c::ENCODE_ACCOUNT_SIZE as usize] =
            [0; c::ENCODE_ACCOUNT_SIZE as usize];

        encode_account(&mut encoded, &ACCOUNT_ID, AccountType::Account);

        assert_eq!(
            encoded,
            [0x81, 0x14, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
        );
    }

    #[wasm_bindgen_test]
    fn enc_signing_pubkey_null() {
        let mut key: [u8; c::ENCODE_SIGNING_PUBKEY_NULL_SIZE as usize] = [255; 35];

        encode_signing_pubkey_null(&mut key);

        assert_eq!(
            key,
            [
                0x73, 0x21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        )
    }
}
