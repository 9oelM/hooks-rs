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

/// Retrieve the parameter value for a named hook parameter
#[inline(always)]
pub fn hook_param<const HOOK_PARAM_LEN: usize>(
    parameter_name: &[u8],
) -> Result<[u8; HOOK_PARAM_LEN]> {
    let mut uninitialized_buffer: [MaybeUninit<u8>; HOOK_PARAM_LEN] = MaybeUninit::uninit_array();
    let buffer: [u8; HOOK_PARAM_LEN] = unsafe {
        let result: Result<u64> = c::hook_param(
            uninitialized_buffer.as_mut_ptr() as u32,
            HOOK_PARAM_LEN as u32,
            parameter_name.as_ptr() as u32,
            parameter_name.len() as u32,
        )
        .into();

        match result {
            Ok(_) => {}
            Err(err) => {
                return Err(err);
            }
        }

        MaybeUninit::array_assume_init(uninitialized_buffer)
    };

    Ok(buffer)
}
