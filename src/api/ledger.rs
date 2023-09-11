use core::mem::{self, MaybeUninit};

use super::*;
use crate::c;

/// Retreive the 20 byte Account ID the Hook is executing on
#[inline(always)]
pub fn hook_account() -> Result<[u8; ACC_ID_LEN]> {
    let mut assume_initialized_buffer: [MaybeUninit<u8>; ACC_ID_LEN] =
        unsafe { MaybeUninit::uninit().assume_init() };

    let buffer: [u8; ACC_ID_LEN] = unsafe {
        let result: Result<u64> = c::hook_account(
            assume_initialized_buffer.as_mut_ptr() as u32,
            ACC_ID_LEN as u32,
        )
        .into();

        match result {
            Ok(_) => {}
            Err(err) => {
                return Err(err);
            }
        }

        mem::transmute::<_, _>(assume_initialized_buffer)
    };

    Ok(buffer)
}

/// Retreive the 32 byte namespace biased SHA512H of the currently executing Hook
#[inline(always)]
pub fn hook_hash(hash: &mut [u8], hook_no: i32) -> Result<u64> {
    buf_write_1arg_i32(hash, hook_no, c::hook_hash)
}

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

/// Retreive the 32 byte namespace biased SHA512H of the last closed ledger
#[inline(always)]
pub fn ledger_last_hash(hash: &mut [u8]) -> Result<u64> {
    buf_write(hash, c::ledger_last_hash)
}

/// Generate a 32 byte nonce for use in an emitted transaction
#[inline(always)]
pub fn nonce(n: &mut [u8]) -> Result<u64> {
    buf_write(n, c::ledger_nonce)
}
