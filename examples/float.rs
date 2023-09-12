//! A hook that accepts any transaction coming through it

#![no_std]
#![no_main]
use hooks_rs::*;

#[no_mangle]
pub extern "C" fn cbak(_: u32) -> i64 {
    0
}

// TODO: more test cases
#[no_mangle]
pub extern "C" fn hook(_: u32) -> i64 {
    max_iter(1);
    // mulratio
    if XFL::one().mulratio(false, 1, 2).unwrap_line_number()
        != XFL::one().mulratio(false, 5, 10).unwrap_line_number()
    {
        rollback(b"", line!().into());
    }
    let decimal_10 = XFL::new(-14, 1000000000000000).unwrap_line_number();
    if XFL::one().mulratio(false, 10, 1).unwrap_line_number() != decimal_10 {
        rollback(b"", line!().into());
    };

    // new
    let plus_1000 = XFL::new(-12, 1000000000000000).unwrap_line_number();
    let plus_999 = XFL::new(-13, 9990000000000000).unwrap_line_number();
    // let plus_998 = XFL::new(-13, 9980000000000000).unwrap_line_number();
    let minus_1000 = (-plus_1000).unwrap_line_number();
    let minus_999 = (-plus_999).unwrap_line_number();
    // let minus_998 = (-plus_998).unwrap_line_number();

    // let _ = trace_float(b"+1000", plus_1000);
    // let _ = trace_float(b"-1000", minus_1000);

    // PartialOrd
    if minus_1000 > minus_999 {
        rollback(b"", line!().into());
    }
    // if minus_999 > minus_998 {
    //     rollback(b"", line!().into());
    // }
    // if minus_1000 > minus_998 {
    //     rollback(b"", line!().into());
    // }
    // let compare_result = unsafe { c::float_compare(plus_1000.0, plus_999.0, c::COMPARE_LESS) };

    // let _ = trace_num(b"compare_result", compare_result);

    // if compare_result == 1 || compare_result < 0 {
    //     rollback(b"", line!().into());
    // }
    if plus_1000 <= plus_999 {
        rollback(b"", line!().into());
    }
    // if plus_999 < plus_998 {
    //     rollback(b"", line!().into());
    // }
    // if plus_1000 < plus_998 {
    //     rollback(b"", line!().into());
    // }

    // // exponent & mantissa
    // if minus_1000.exponent() != -12 {
    //     rollback(b"", line!().into());
    // }
    // if minus_1000.mantissa() != 1000000000000000 {
    //     rollback(b"", line!().into());
    // }
    // if minus_999.exponent() != -13 {
    //     rollback(b"", line!().into());
    // }
    // if minus_999.mantissa() != 9990000000000000 {
    //     rollback(b"", line!().into());
    // }
    // if minus_998.exponent() != -13 {
    //     rollback(b"", line!().into());
    // }
    // if minus_998.mantissa() != 9980000000000000 {
    //     rollback(b"", line!().into());
    // }

    // // to_int64
    // let approx_pi = XFL::new(-15, 3140000000000000).unwrap_line_number();

    // if 3 != approx_pi.to_int64(0, false).unwrap_line_number() {
    //     rollback(b"", line!().into());
    // }

    // // negation
    // // 1.220111606619773e+32
    // let a = XFL::new(17, 1220111606619773).unwrap_line_number();
    // let expected_negated_enclosing: i64 = 2054861541687565949;
    // if (-a).unwrap_line_number().0 != expected_negated_enclosing {
    //     rollback(b"", line!().into());
    // }

    // // multiplication
    // // 3.845483684710632e-29
    // let a = XFL::new(-44, 3845483684710632).unwrap_line_number();
    // // 3.604275125235076e-32
    // let b = XFL::new(-47, 3604275125235076).unwrap_line_number();
    // // 1.386018118929985e-60
    // let expected_multiplication_enclosing = 5009388803754921537;
    // if (a * b).unwrap_line_number().0 != expected_multiplication_enclosing {
    //     rollback(b"", line!().into());
    // }

    // // division
    // // 4.676501568852917e+23
    // let a = XFL::new(8, 4676501568852917).unwrap_line_number();
    // // 2.470512232954799e+26
    // let b = XFL::new(11, 2470512232954799).unwrap_line_number();
    // // 0.00189292791449152
    // let expected_multiplication_enclosing = 5009388803754921537;
    // if (a / b).unwrap_line_number().0 != expected_multiplication_enclosing {
    //     rollback(b"", line!().into());
    // }

    // // addition
    // // 123123123123123
    // let a = XFL::new(-1, 1231231231231230).unwrap_line_number();
    // // 55555555
    // let b = XFL::new(-8, 5555555500000000).unwrap_line_number();
    // // 123123178678678
    // let expected_addition_enclosing = 6342299507124445148;
    // if (a + b).unwrap_line_number().0 != expected_addition_enclosing {
    //     rollback(b"", line!().into());
    // }

    // Accept all
    accept(b"", 0);
}
