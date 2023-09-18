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
    etxn_reserve(1).unwrap_line_number();

    let otxn_account = match otxn_field::<ACC_ID_LEN>(FieldId::Account) {
        Ok(account) => account,
        Err(err) => {
            rollback(b"could not get otxn account", err.into());
        }
    };
    let xrp_payment_txn_builder = XrpPaymentBuilder::new(1000, &otxn_account, 0, 0);
    let xrp_payment_txn = match xrp_payment_txn_builder.build() {
        Ok(txn) => txn,
        Err(err) => {
            rollback(b"could not build xrp payment txn", err.into());
        }
    };
    let _ = trace(b"txn", &xrp_payment_txn, DataRepr::AsHex);
    let txn_hash = match emit(&xrp_payment_txn) {
        Ok(hash) => hash,
        Err(err) => {
            rollback(b"could not emit xrp payment txn", err.into());
        }
    };

    let _ = trace(b"emit", &txn_hash, DataRepr::AsHex);

    accept(&txn_hash, 0);
}
