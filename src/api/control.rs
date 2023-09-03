use crate::c;
/// Guard function
///
/// Each time a loop appears in your code a call to this must be
/// the first branch instruction in wasm binary after the beginning of the loop.
/// In order to achieve this in Rust use the `while` loop with expression block.
///
/// # Example
///
/// ```no_run
/// let mut i = 0;
/// while {
///     _g(UNIQUE_GUARD_ID, MAXITER + 1);
///     i < MAXITER
/// } {
///     // your code
///     i += 1;
/// }
/// ```
#[cfg(not(doctest))]
#[inline(always)]
pub fn _g(id: u32, maxiter: u32) {
    unsafe {
        c::_g(id, maxiter);
    }
}

/// Accept the originating transaction and commit any changes the hook made
#[inline(always)]
pub fn accept(msg: &[u8], error_code: i64) -> ! {
    unsafe {
        c::accept(msg.as_ptr() as u32, msg.len() as u32, error_code);
        core::hint::unreachable_unchecked()
    }
}

/// Reject the originating transaction and discard any changes the hook made
#[inline(always)]
pub fn rollback(msg: &[u8], error_code: i64) -> ! {
    unsafe {
        c::rollback(msg.as_ptr() as u32, msg.len() as u32, error_code);
        core::hint::unreachable_unchecked()
    }
}
