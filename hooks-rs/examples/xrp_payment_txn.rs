//! This hook sends a payment transaction to the same account that initiated the hook.

#![no_std]
#![no_main]

use hooks_rs::*;

#[no_mangle]
pub extern "C" fn cbak(_: u32) -> i64 {
    0
}

#[no_mangle]
pub extern "C" fn hook(_: u32) -> i64 {
    // Every hook needs to import guard function
    // and use it at least once
    max_iter(1);
    etxn_reserve(1).unwrap_line_number();

    let otxn_account = match otxn_field::<ACC_ID_LEN>(FieldId::Account) {
        Ok(account) => account,
        Err(err) => {
            rollback(b"could not get otxn account", err.into());
        }
    };
    let xrp_payment_txn_builder = XrpPaymentBuilder::new(1000, &otxn_account, 0, 0);
    let mut xrp_payment_txn_buffer = XrpPaymentBuilder::uninit_buffer();
    match xrp_payment_txn_builder.build(&mut xrp_payment_txn_buffer) {
        Ok(_) => {}
        Err(err) => {
            rollback(b"could not build xrp payment txn", err.into());
        }
    };
    let txn_hash = match emit(&xrp_payment_txn_buffer) {
        Ok(hash) => hash,
        Err(err) => {
            rollback(b"could not emit xrp payment txn", err.into());
        }
    };

    accept(&txn_hash, 0);
}
