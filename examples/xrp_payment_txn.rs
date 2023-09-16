#![no_std]
#![no_main]
#![no_builtins]

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

    let otxn_account = otxn_field::<ACC_ID_LEN>(FieldId::Account).unwrap_line_number();
    let xrp_payment_txn_builder = XrpPaymentBuilder::new(1000, &otxn_account, 0, 0);
    let xrp_payment_txn = xrp_payment_txn_builder.build().unwrap_line_number();
    let _ = emit(&xrp_payment_txn).unwrap_line_number();

    accept(b"", 0);
}
