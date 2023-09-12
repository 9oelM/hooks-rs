#![no_std]
#![no_main]

use hooks_rs::*;

#[no_mangle]
pub extern "C" fn cbak(_: u32) -> i64 {
    0
}

#[no_mangle]
pub extern "C" fn hook(_: u32) -> i64 {
    max_iter(1);

    const STATE_KEY: &[u8; 14] = b"same same same";
    const STATE_VALUE: &[u8; 14] = b"same same same";

    if STATE_KEY == STATE_VALUE {
        accept(b"", 0);
    } else {
        rollback(b"", -1);
    }
}
