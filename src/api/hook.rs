use core::mem::MaybeUninit;

use super::*;
use crate::c;

/// Retreive the 20 byte Account ID the Hook is executing on
///
/// # Example
/// ```
/// let hook_account = match hook_account() {
///     Ok(acc) => acc,
///     Err(err) => {
///         rollback(b"hook_account.rs: hook_account() failed.", err.into());
///     }
/// };
///
/// accept(&hook_account, 0);
/// ```
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
///
/// # Example
/// ```
/// match hook_param::<HOOK_PARAM_LEN>(b"param test") {
///     Ok(param) => accept(&param, 0),
///     Err(err) => rollback(b"cannot find hook param", err.into()),
/// }
/// ```
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
