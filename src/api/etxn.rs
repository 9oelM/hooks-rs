use crate::c;

use super::*;

/// Get the burden of a hypothetically emitted transaction
#[inline(always)]
pub fn etxn_burden() -> i64 {
    unsafe { c::etxn_burden() }
}

/// Produce an sfEmitDetails suitable for a soon-to-be emitted transaction
#[inline(always)]
pub fn etxn_details(emitdet: &mut [u8]) -> Result<u64> {
    buf_write(emitdet, c::etxn_details)
}

/// Estimate the required fee for a txn to be emitted successfully
#[inline(always)]
pub fn etxn_fee_base(tx_blob: &mut [u8]) -> Result<u64> {
    buf_write(tx_blob, c::etxn_fee_base)
}

/// Estimate the required fee for a txn to be emitted successfully
#[inline(always)]
pub fn etxn_reserve(count: u32) -> Result<u64> {
    api_1arg_call(count, c::etxn_reserve)
}

/// Get the generation of a hypothetically emitted transaction
#[inline(always)]
pub fn etxn_generation() -> i64 {
    unsafe { c::etxn_generation() }
}

/// Emit a new transaction from the hook
#[inline(always)]
pub fn emit(hash: &mut [u8], tx_buf: &[u8]) -> Result<u64> {
    buf_write_read(hash, tx_buf, c::emit)
}
