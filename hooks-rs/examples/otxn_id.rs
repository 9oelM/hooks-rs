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

    match otxn_id(OtxnIdFlag::Zero) {
        Ok(otxn_id) => {
            accept(&otxn_id, 0);
        }
        Err(_) => {
            rollback(b"could not get otxn id", line!().into());
        }
    };
}
