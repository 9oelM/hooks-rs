use crate::c;

// Safe to be modified globally because wasm is guaranteed to run in a single thread
static mut GUARD_ID: u32 = 0;

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
pub fn _g(id: u32, max_iter: u32) {
    unsafe {
        c::_g(id, max_iter);
    }
}

/// Instead of having to pass the `GUARD_ID` parameter to every call to `_g`,
/// you can use this function to generate a unique `GUARD_ID` for each call
/// automatically.
///
/// # Example
///
/// ```no_run
/// let mut i = 0;
/// while {
///     max_iter(MAXITER + 1);
///     i < MAXITER
/// } {
///     // your code
///     i += 1;
/// }
/// ```
#[inline(always)]
pub fn max_iter(max_iter: u32) {
    unsafe {
        GUARD_ID += 1;
        _g(GUARD_ID, max_iter);
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
