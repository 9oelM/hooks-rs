//! A hook that accepts any transaction coming through it

#![no_std]
#![no_main]

use hooks_rs::*;

const GUARD_ID_1: u32 = line!();

#[no_mangle]
pub extern "C" fn cbak(_: i64) -> i64 {
    0
}

#[no_mangle]
pub extern "C" fn hook(_: i64) -> i64 {
    // Every hook needs to import guard function
    // and use it at least once
    _g(GUARD_ID_1, 1);

    // Tracing when compiling in debug mode
    #[cfg(debug_assertions)]
    let _ = trace(b"Accept: called", b"", DataRepr::AsUTF8);

    // Accept all
    accept(b"Accept: accepted", 0)
}
