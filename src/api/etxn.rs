use core::mem::MaybeUninit;

use crate::c;

use super::*;

/// Get the burden of a hypothetically emitted transaction
#[inline(always)]
pub fn etxn_burden() -> i64 {
    unsafe { c::etxn_burden() }
}

/// Produce an sfEmitDetails suitable for a soon-to-be emitted transaction
#[inline(always)]
pub fn etxn_details() -> Result<[u8; EMIT_DETAILS_SIZE]> {
    init_buffer_mut(|buffer_mut_ptr: *mut MaybeUninit<u8>| {
        let result: Result<u64> =
            unsafe { c::etxn_details(buffer_mut_ptr as u32, EMIT_DETAILS_SIZE as u32).into() };

        result
    })
}

/// Estimate the required fee for a txn to be emitted successfully
#[inline(always)]
pub fn etxn_fee_base(tx_blob: &[u8]) -> i64 {
    unsafe { c::etxn_fee_base(tx_blob.as_ptr() as u32, tx_blob.len() as u32) }
}

/// Generate a 32 byte nonce for use in an emitted transaction
#[inline(always)]
pub fn etxn_nonce() -> Result<[u8; NONCE_LEN]> {
    init_buffer_mut(|buffer_mut_ptr: *mut MaybeUninit<u8>| {
        let result: Result<u64> =
            unsafe { c::etxn_nonce(buffer_mut_ptr as u32, NONCE_LEN as u32).into() };

        result
    })
}

/// Estimate the required fee for a txn to be emitted successfully
#[inline(always)]
pub fn etxn_reserve(count: u32) -> Result<u64> {
    unsafe { c::etxn_reserve(count).into() }
}

/// Get the generation of a hypothetically emitted transaction
#[inline(always)]
pub fn etxn_generation() -> i64 {
    unsafe { c::etxn_generation() }
}

/// Emit a new transaction from the hook and return the 32-bytes long txn hash
#[inline(always)]
pub fn emit(tx: &[u8]) -> Result<[u8; HASH_LEN]> {
    let func = |buffer_mut_ptr: *mut MaybeUninit<u8>| {
        let result: Result<u64> = unsafe {
            c::emit(
                buffer_mut_ptr as u32,
                HASH_LEN as u32,
                tx.as_ptr() as u32,
                tx.len() as u32,
            )
            .into()
        };

        result
    };

    init_buffer_mut(func)
}
