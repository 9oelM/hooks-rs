use super::*;
use crate::c;

/// Fetch the fee base of the current ledger
#[inline(always)]
pub fn fee_base() -> i64 {
    unsafe { c::fee_base() }
}

/// Fetch the current ledger sequence number
#[inline(always)]
pub fn ledger_seq() -> i64 {
    unsafe { c::ledger_seq() }
}

/// Fetch the last time the ledger was closed
#[inline(always)]
pub fn ledger_last_time() -> i64 {
    unsafe { c::ledger_last_time() }
}

/// Retreive the 32 byte namespace biased SHA512H of the last closed ledger
#[inline(always)]
pub fn ledger_last_hash(hash: &mut [u8]) -> Result<u64> {
    buf_write(hash, c::ledger_last_hash)
}

/// Generate a 32 byte nonce for use in an emitted transaction
#[inline(always)]
pub fn ledger_nonce(n: &mut [u8]) -> Result<u64> {
    buf_write(n, c::ledger_nonce)
}
