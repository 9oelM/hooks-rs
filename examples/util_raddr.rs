#![feature(maybe_uninit_uninit_array)]
#![no_std]
#![no_main]

use core::mem::MaybeUninit;

use hooks_rs::*;

#[no_mangle]
pub extern "C" fn cbak(_: u32) -> i64 {
    0
}

#[no_mangle]
pub extern "C" fn hook(_: u32) -> i64 {
    max_iter(1);

    // d9 88 38 fb fe 5f 5e c2 65 b8 b6 cf f7 f0 2d df 59 05 ea cb
    let mut uninitialized_account_id: [MaybeUninit<u8>; 20] = MaybeUninit::uninit_array();
    uninitialized_account_id[0].write(0xd9);
    uninitialized_account_id[1].write(0x88);
    uninitialized_account_id[2].write(0x38);
    uninitialized_account_id[3].write(0xfb);
    uninitialized_account_id[4].write(0xfe);
    uninitialized_account_id[5].write(0x5f);
    uninitialized_account_id[6].write(0x5e);
    uninitialized_account_id[7].write(0xc2);
    uninitialized_account_id[8].write(0x65);
    uninitialized_account_id[9].write(0xb8);
    uninitialized_account_id[10].write(0xb6);
    uninitialized_account_id[11].write(0xcf);
    uninitialized_account_id[12].write(0xf7);
    uninitialized_account_id[13].write(0xf0);
    uninitialized_account_id[14].write(0x2d);
    uninitialized_account_id[15].write(0xdf);
    uninitialized_account_id[16].write(0x59);
    uninitialized_account_id[17].write(0x05);
    uninitialized_account_id[18].write(0xea);
    uninitialized_account_id[19].write(0xcb);

    let account_id = unsafe {
        uninitialized_account_id
            .as_ptr()
            .cast::<[u8; 20]>()
            .read_volatile()
    };
    let raddr = match util_raddr(&account_id) {
        Ok(raddr) => raddr,
        Err(err) => {
            accept(b"util_raddr failed.", err.into());
        }
    };

    accept(&raddr, 0);
}
