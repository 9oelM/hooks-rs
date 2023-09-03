use crate::c;

use super::*;

/// Get the burden of the originating transaction
#[inline(always)]
pub fn otxn_burden() -> i64 {
    unsafe { c::otxn_burden() }
}

/// Serialize and output a field from the originating transaction
#[inline(always)]
pub fn otxn_field(accid: &mut [u8], field_id: FieldId) -> Result<u64> {
    buf_write_1arg(accid, field_id as _, c::otxn_field)
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
