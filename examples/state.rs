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

    const STATE_KEY_LEN: usize = 15;
    const STATE_KEY: &[u8; STATE_KEY_LEN] = b"hello world key";
    const STATE_VALUE_LEN: usize = 15;
    const STATE_VALUE: &[u8; STATE_VALUE_LEN] = b"hello world val";

    match state_set(STATE_VALUE.as_ref(), STATE_KEY) {
        Ok(_) => {}
        Err(_) => {
            rollback(b"could not set state", -1);
        }
    };
    match state::<STATE_VALUE_LEN>(STATE_KEY.as_ref()) {
        Ok(data) => {
            accept(&data, 0);
        }
        Err(_) => {
            rollback(b"could not get state", -1);
        }
    };
}
