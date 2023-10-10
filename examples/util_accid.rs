#![feature(maybe_uninit_uninit_array)]
#![no_std]
#![no_main]

use core::mem::MaybeUninit;

use hooks_rs::*;

const EXPECTED_R_ADDR_LEN: usize = 34;
// [114, 76, 113, 85, 70, 89, 71, 76, 77, 66, 83, 57, 106, 70, 54, 51, 105, 82, 107, 97, 100, 118, 117, 51, 99, 84, 105, 120, 97, 100, 82, 84, 100, 51]
// "rLqUFYGLMBS9jF63iRkadvu3cTixadRTd3"

#[no_mangle]
pub extern "C" fn cbak(_: u32) -> i64 {
    0
}

#[no_mangle]
pub extern "C" fn hook(_: u32) -> i64 {
    max_iter(1);

    let mut uninit_r_addr: [MaybeUninit<u8>; EXPECTED_R_ADDR_LEN] = MaybeUninit::uninit_array();
    unsafe {
        let mut pos_0 = MaybeUninit::uninit();
        let mut pos_1 = MaybeUninit::uninit();
        let mut pos_2 = MaybeUninit::uninit();
        let mut pos_3 = MaybeUninit::uninit();
        let mut pos_4 = MaybeUninit::uninit();
        let mut pos_5 = MaybeUninit::uninit();
        let mut pos_6 = MaybeUninit::uninit();
        let mut pos_7 = MaybeUninit::uninit();
        let mut pos_9 = MaybeUninit::uninit();
        let mut pos_10 = MaybeUninit::uninit();
        let mut pos_11 = MaybeUninit::uninit();
        let mut pos_12 = MaybeUninit::uninit();
        let mut pos_13 = MaybeUninit::uninit();
        let mut pos_14 = MaybeUninit::uninit();
        let mut pos_15 = MaybeUninit::uninit();
        let mut pos_16 = MaybeUninit::uninit();
        let mut pos_17 = MaybeUninit::uninit();
        let mut pos_18 = MaybeUninit::uninit();
        let mut pos_19 = MaybeUninit::uninit();
        let mut pos_20 = MaybeUninit::uninit();
        let mut pos_21 = MaybeUninit::uninit();
        let mut pos_22 = MaybeUninit::uninit();
        let mut pos_23 = MaybeUninit::uninit();
        let mut pos_24 = MaybeUninit::uninit();
        let mut pos_25 = MaybeUninit::uninit();
        let mut pos_26 = MaybeUninit::uninit();
        let mut pos_27 = MaybeUninit::uninit();
        let mut pos_28 = MaybeUninit::uninit();
        let mut pos_29 = MaybeUninit::uninit();
        let mut pos_30 = MaybeUninit::uninit();
        let mut pos_31 = MaybeUninit::uninit();
        let mut pos_32 = MaybeUninit::uninit();
        let mut pos_33 = MaybeUninit::uninit();
        let mut pos_34 = MaybeUninit::uninit();

        pos_0.write(114);
        pos_1.write(76);
        pos_2.write(113);
        pos_3.write(85);
        pos_4.write(70);
        pos_5.write(89);
        pos_6.write(71);
        pos_7.write(76);
        pos_9.write(77);
        pos_10.write(66);
        pos_11.write(83);
        pos_12.write(57);
        pos_13.write(106);
        pos_14.write(70);
        pos_15.write(54);
        pos_16.write(51);
        pos_17.write(105);
        pos_18.write(82);
        pos_19.write(107);
        pos_20.write(97);
        pos_21.write(100);
        pos_22.write(118);
        pos_23.write(117);
        pos_24.write(51);
        pos_25.write(99);
        pos_26.write(84);
        pos_27.write(105);
        pos_28.write(120);
        pos_29.write(97);
        pos_30.write(100);
        pos_31.write(82);
        pos_32.write(84);
        pos_33.write(100);
        pos_34.write(51);

        uninit_r_addr.as_mut_ptr().add(0).write(pos_0);
        uninit_r_addr.as_mut_ptr().add(1).write(pos_1);
        uninit_r_addr.as_mut_ptr().add(2).write(pos_2);
        uninit_r_addr.as_mut_ptr().add(3).write(pos_3);
        uninit_r_addr.as_mut_ptr().add(4).write(pos_4);
        uninit_r_addr.as_mut_ptr().add(5).write(pos_5);
        uninit_r_addr.as_mut_ptr().add(6).write(pos_6);
        uninit_r_addr.as_mut_ptr().add(7).write(pos_7);
        uninit_r_addr.as_mut_ptr().add(9).write(pos_9);
        uninit_r_addr.as_mut_ptr().add(10).write(pos_10);
        uninit_r_addr.as_mut_ptr().add(11).write(pos_11);
        uninit_r_addr.as_mut_ptr().add(12).write(pos_12);
        uninit_r_addr.as_mut_ptr().add(13).write(pos_13);
        uninit_r_addr.as_mut_ptr().add(14).write(pos_14);
        uninit_r_addr.as_mut_ptr().add(15).write(pos_15);
        uninit_r_addr.as_mut_ptr().add(16).write(pos_16);
        uninit_r_addr.as_mut_ptr().add(17).write(pos_17);
        uninit_r_addr.as_mut_ptr().add(18).write(pos_18);
        uninit_r_addr.as_mut_ptr().add(19).write(pos_19);
        uninit_r_addr.as_mut_ptr().add(20).write(pos_20);
        uninit_r_addr.as_mut_ptr().add(21).write(pos_21);
        uninit_r_addr.as_mut_ptr().add(22).write(pos_22);
        uninit_r_addr.as_mut_ptr().add(23).write(pos_23);
        uninit_r_addr.as_mut_ptr().add(24).write(pos_24);
        uninit_r_addr.as_mut_ptr().add(25).write(pos_25);
        uninit_r_addr.as_mut_ptr().add(26).write(pos_26);
        uninit_r_addr.as_mut_ptr().add(27).write(pos_27);
        uninit_r_addr.as_mut_ptr().add(28).write(pos_28);
        uninit_r_addr.as_mut_ptr().add(29).write(pos_29);
        uninit_r_addr.as_mut_ptr().add(30).write(pos_30);
        uninit_r_addr.as_mut_ptr().add(31).write(pos_31);
        uninit_r_addr.as_mut_ptr().add(32).write(pos_32);
        uninit_r_addr.as_mut_ptr().add(33).write(pos_33);
        uninit_r_addr.as_mut_ptr().add(34).write(pos_34);
    }
    let r_addr = unsafe {
        uninit_r_addr
            .as_ptr()
            .cast::<[u8; EXPECTED_R_ADDR_LEN]>()
            .read_volatile()
    };

    let acc_id = match util_accid(&r_addr) {
        Ok(acc_id) => acc_id,
        Err(err) => {
            rollback(b"util_accid failed.", err.into());
        }
    };

    accept(&acc_id, 0);
}
