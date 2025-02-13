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

    let hook_hash = hook_hash(HookNumber::CurrentHook).unwrap_line_number();

    accept(&hook_hash, 0);
}
