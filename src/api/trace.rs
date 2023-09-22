use core::mem::size_of_val;

use crate::c;

use super::*;

/// Write the contents of a buffer to the XRPLD trace log
///
/// # Example
///
/// The first argument will be shown on the left side of the log
/// and the second argument will be shown on the right side of the log.
///
/// Usually you would want to use the first argument to specify what the
/// data is and the second argument to specify the data itself.
/// ```
/// let _ = trace(b"left", b"right", DataRepr::AsUTF8);
/// ```
#[inline(always)]
pub fn trace(msg: &[u8], data: &[u8], data_repr: DataRepr) -> Result<u64> {
    let res = unsafe {
        c::trace(
            msg.as_ptr() as u32,
            size_of_val(msg) as u32,
            data.as_ptr() as u32,
            size_of_val(data) as u32,
            data_repr as _,
        )
    };

    res.into()
}

/// Write the contents of a slot to the XRPLD trace log
#[inline(always)]
pub fn trace_slot(msg: &[u8], slot: u32) -> Result<u64> {
    let res = unsafe { c::trace_slot(msg.as_ptr() as u32, size_of_val(msg) as u32, slot) };

    res.into()
}

/// Write an integer to the XRPLD trace log
///
/// # Example
/// ```
/// let _ = trace_num(b"my number", 42);
/// ```
#[inline(always)]
pub fn trace_num(msg: &[u8], number: i64) -> Result<u64> {
    let res = unsafe { c::trace_num(msg.as_ptr() as u32, size_of_val(msg) as u32, number) };

    res.into()
}

/// Write a XFL float to the XRPLD trace log
///
/// # Example
/// ```
/// let _ = trace_float(b"my float", XFL(42.0));
/// ```
#[inline(always)]
pub fn trace_float(msg: &[u8], float: XFL) -> Result<u64> {
    let res = unsafe { c::trace_float(msg.as_ptr() as u32, size_of_val(msg) as u32, float.0) };

    res.into()
}
