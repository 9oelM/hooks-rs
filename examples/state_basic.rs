#![no_std]
#![no_main]

use hooks_rs::*;

const GUARD_ID_1: u32 = line!();

#[no_mangle]
pub extern "C" fn cbak(_: u32) -> i64 {
    0
}

#[no_mangle]
pub extern "C" fn hook(_: u32) -> i64 {
    const COUNT_STATE_KEY: &[u8; 5] = b"count";
    // u64
    const STATE_VALUE_LEN: usize = 8;
    _g(1, 1);

    let otxn_account = match otxn_field::<ACC_ID_LEN>(FieldId::Account) {
        Ok(data) => data,
        Err(_) => {
            rollback(b"could not get otxn account", -1);
        }
    };
    let hook_account = match hook_account() {
        Ok(data) => data,
        Err(_) => {
            rollback(b"could not get hook account", -1);
        }
    };
    if !is_buffer_equal::<GUARD_ID_1>(otxn_account.as_ref(), hook_account.as_ref()) {
        rollback(b"otxn account must be hook account", -1);
    }
    set_count(1);
    let count = get_count();
    set_count(count + 1);
    let count_again = get_count();

    accept(count_again.to_be_bytes().as_ref(), 0);
}

#[inline(always)]
fn get_count() -> u64 {
    const COUNT_STATE_KEY: &[u8; 5] = b"count";
    // u64
    const STATE_VALUE_LEN: usize = 8;
    match state::<STATE_VALUE_LEN>(COUNT_STATE_KEY.as_ref()) {
        Ok(data) => {
            trace(b"count", data.as_ref(), DataRepr::AsHex);
            u64::from_be_bytes(data)
        }
        Err(err) => {
            rollback(b"could not get count state", -1);
        }
    }
}

#[inline(always)]
fn set_count(count: u64) {
    const COUNT_STATE_KEY: &[u8; 5] = b"count";
    // u64
    const STATE_VALUE_LEN: usize = 8;
    match state_set(count.to_be_bytes().as_ref(), COUNT_STATE_KEY) {
        Ok(_) => {}
        Err(_) => {
            rollback(b"could not set state", -1);
        }
    };
}
