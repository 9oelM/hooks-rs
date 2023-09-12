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

    const A: ComparableArray<u8, 14> = ComparableArray::new(*b"same same same");
    const B: ComparableArray<u8, 14> = ComparableArray::new(*b"same same same");

    if A == B {
        accept(b"", 0);
    } else {
        rollback(b"", -1);
    }
}
