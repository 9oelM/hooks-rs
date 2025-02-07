#![no_std]
#![no_main]

use hooks_rs::*;

#[no_mangle]
pub extern "C" fn cbak(_: u32) -> i64 {
    0
}

#[no_mangle]
pub extern "C" fn hook(_: u32) -> i64 {
    _g(1, 1);

    // Check https://docs.xahau.network/technical/hooks-c-functions/originating-transaction/otxn_type#known-transaction-types
    let otxn_type = otxn_type();
    accept(b"", otxn_type);
}
