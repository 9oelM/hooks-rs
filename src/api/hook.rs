use core::mem::MaybeUninit;

use super::*;
use crate::c;

/// Retreive the 20 byte Account ID the Hook is executing on
#[inline(always)]
pub fn hook_account() -> Result<[u8; ACC_ID_LEN]> {
    let func = |buffer_mut_ptr: *mut MaybeUninit<u8>| {
        let result: Result<u64> =
            unsafe { c::hook_account(buffer_mut_ptr as u32, ACC_ID_LEN as u32).into() };

        result
    };

    init_buffer_mut(func)
}

/// Retrieve the parameter value for a named hook parameter
#[inline(always)]
pub fn hook_param<const HOOK_PARAM_LEN: usize>(
    parameter_name: &[u8],
) -> Result<[u8; HOOK_PARAM_LEN]> {
    let func = |buffer_mut_ptr: *mut MaybeUninit<u8>| {
        let result: Result<u64> = unsafe {
            c::hook_param(
                buffer_mut_ptr as u32,
                HOOK_PARAM_LEN as u32,
                parameter_name.as_ptr() as u32,
                parameter_name.len() as u32,
            )
            .into()
        };

        result
    };

    init_buffer_mut(func)
}
