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

    let account_field = match otxn_field::<ACC_ID_LEN>(FieldId::Account) {
        Ok(field) => field,
        Err(_) => rollback(b"could not get originating txn's account field", -1),
    };

    accept(&account_field, 0);
}
