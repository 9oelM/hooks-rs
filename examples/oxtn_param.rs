#![no_std]
#![no_main]

use hooks_rs::*;

// At least 32 bytes required
const OTXN_PARAM_LEN: usize = 32;

#[no_mangle]
pub extern "C" fn cbak(_: u32) -> i64 {
    0
}

#[no_mangle]
pub extern "C" fn hook(_: u32) -> i64 {
    _g(1, 1);

    match otxn_param::<OTXN_PARAM_LEN>(b"param test") {
        Ok(param) => accept(&param, 0),
        Err(err) => rollback(b"cannot find hook param", err.into()),
    }
}
