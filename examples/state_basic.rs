#![no_std]
#![no_main]

use hooks_rs::*;

const STATE_VALUE_LEN: usize = 8;

#[no_mangle]
pub extern "C" fn cbak(_: u32) -> i64 {
    0
}

#[no_mangle]
pub extern "C" fn hook(_: u32) -> i64 {
    max_iter(1);

    let otxn_account = match otxn_field::<ACC_ID_LEN>(FieldId::Account) {
        Ok(data) => data,
        Err(_) => {
            rollback(b"could not get otxn account", -1);
        }
    };
    let hook_account = match hook_account() {
        Ok(data) => data,
        Err(_) => {
            rollback(b"could not get hook account", -2);
        }
    };
    {
        set_count(1, &otxn_account);
        let count = get_count(&otxn_account);
        set_count(count + 1, &otxn_account);
        let _count_again = get_count(&otxn_account);
    }
    {
        set_count(1, &hook_account);
        let count = get_count(&hook_account);
        set_count(count + 1, &hook_account);
        let count_again = get_count(&hook_account);

        accept(count_again.to_be_bytes().as_ref(), 0);
    }
}

#[inline(always)]
fn get_count(key: &[u8; ACC_ID_LEN]) -> u64 {
    match state::<STATE_VALUE_LEN>(key.as_ref()) {
        Ok(data) => u64::from_be_bytes(data),
        Err(_err) => {
            rollback(b"could not get count state", -1);
        }
    }
}

#[inline(always)]
fn set_count(count: u64, key: &[u8; ACC_ID_LEN]) {
    match state_set(count.to_be_bytes().as_ref(), key.as_ref()) {
        Ok(_) => {}
        Err(_) => {
            rollback(b"could not set state", -1);
        }
    };
}
