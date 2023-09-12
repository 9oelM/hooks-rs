use core::mem::MaybeUninit;

use crate::c;

use super::*;

/// Get the burden of the originating transaction
#[inline(always)]
pub fn otxn_burden() -> i64 {
    unsafe { c::otxn_burden() }
}

// TODO: relate FieldId and BUFFER_LEN so user doesn't need to find the right BUFFER_LEN
// when using FieldId
/// Serialize and output a field from the originating transaction
#[inline(always)]
pub fn otxn_field<const BUFFER_LEN: usize>(field_id: FieldId) -> Result<[u8; BUFFER_LEN]> {
    let func = |buffer_mut_ptr: *mut MaybeUninit<u8>| {
        let result: Result<u64> = unsafe {
            c::otxn_field(buffer_mut_ptr as u32, BUFFER_LEN as u32, field_id as u32).into()
        };

        result
    };

    init_buffer_mut(func)
}

/// Output a field from the originating transaction as a human readable string
#[inline(always)]
pub fn otxn_field_txt(acctxt: &mut [u8], field_id: FieldId) -> Result<u64> {
    buf_write_1arg(acctxt, field_id as _, c::otxn_field_txt)
}

/// Get the generation of the originating transaction
#[inline(always)]
pub fn otxn_generation() -> i64 {
    unsafe { c::otxn_generation() }
}

/// Output the canonical hash of the originating transaction
#[inline(always)]
pub fn otxn_id(hash: &mut [u8], flags: u32) -> Result<u64> {
    buf_write_1arg(hash, flags, c::otxn_id)
}

/// Get the Transaction Type of the originating transaction
#[inline(always)]
pub fn otxn_type() -> i64 {
    unsafe { c::otxn_type() }
}

/// Load the originating transaction into a slot
#[inline(always)]
pub fn otxn_slot(slot_no: u32) -> Result<u64> {
    api_1arg_call(slot_no, c::otxn_slot)
}

/// Retrieve the parameter value for a named Invoke transaction parameter
#[inline(always)]
pub fn otxn_param<const PARAM_LEN: usize>(parameter_name: &[u8]) -> Result<[u8; PARAM_LEN]> {
    let func = |buffer_mut_ptr: *mut MaybeUninit<u8>| {
        let result: Result<u64> = unsafe {
            c::otxn_param(
                buffer_mut_ptr as u32,
                PARAM_LEN as u32,
                parameter_name.as_ptr() as u32,
                parameter_name.len() as u32,
            )
            .into()
        };

        result
    };

    init_buffer_mut(func)
}
