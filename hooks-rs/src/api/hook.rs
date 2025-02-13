use core::mem::MaybeUninit;

use super::*;
use crate::c;

/// Meant to be used as an argument to `hook_hash` to specify the hook number.
#[derive(Copy, Clone)]
pub enum HookNumber {
    /// The currently executing hook
    CurrentHook,
    /// The position in the hook chain the hook is located at,
    Custom(i32),
}

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

/// Returns the position in the hook chain the currently executing hook occupies.
/// Returns the position in the chain the currently executing hook occupies. The first position is 0.
///
/// # Example
/// ```
/// // hook_pos is 0 for the first hook in the chain
/// let hook_pos = hook_pos();
/// ```
#[inline(always)]
pub fn hook_pos() -> i64 {
    unsafe { c::hook_pos() }
}

/// Retreive the 32 byte namespace biased SHA512H of the currently executing Hook
///
/// # Example
/// ```
/// let hook_hash = hook_hash(HookNumber::CurrentHook);
/// ```
#[inline(always)]
pub fn hook_hash(hook_number: HookNumber) -> Result<[u8; HOOK_HASH_LEN]> {
    let func = |buffer_mut_ptr: *mut MaybeUninit<u8>| {
        let result: Result<u64> = unsafe {
            c::hook_hash(
                buffer_mut_ptr as u32,
                HOOK_HASH_LEN as u32,
                hook_number.into(),
            )
            .into()
        };

        result
    };

    init_buffer_mut(func)
}

impl From<HookNumber> for i32 {
    fn from(hook_no: HookNumber) -> i32 {
        match hook_no {
            HookNumber::CurrentHook => -1,
            HookNumber::Custom(hook_no) => hook_no,
        }
    }
}
