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

    let hook_account = match hook_account() {
        Ok(acc) => acc,
        Err(err) => {
            rollback(b"hook_account.rs: hook_account() failed.", err.into());
        }
    };

    accept(&hook_account, 0);
}
