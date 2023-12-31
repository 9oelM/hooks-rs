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

    const A: &[u8; 14] = b"same same same";
    const B: &[u8; 14] = b"same same same";

    const COMPARABLE_A: ComparableArray<u8, 14> = ComparableArray::new(A);
    const COMPARABLE_B: ComparableArray<u8, 14> = ComparableArray::new(B);
    const COMPARABLE_C: ComparableArray<u8, 14> = ComparableArray::new(b"diff diff diff");

    // This syntax is only allowed for ComparableArray
    if COMPARABLE_A != COMPARABLE_B {
        // 14
        rollback(b"COMPARABLE_A != COMPARABLE_B", -1)
    }
    if COMPARABLE_A == COMPARABLE_C {
        rollback(b"COMPARABLE_A == COMPARABLE_C", -1)
    }

    // This is also a valid syntax. In fact, the equality operator
    // for ComparableArray is overloaded with a function that calls is_buffer_equal.
    if !is_buffer_equal(A, B) {
        // 14
        rollback(b"!is_buffer_equal(A, B)", -1)
    }

    // This is a primitive form of comparison.
    let mut i = 0;
    while {
        max_iter(A.len() as u32 + 1); // 14
        i < A.len()
    } {
        if A[i] != B[i] {
            rollback(b"max_iter", -1)
        }
        i += 1;
    }

    // This is the most primitive form of comparison.
    // you will hardly need this.
    const GUARD_ID: u32 = line!();

    let mut i = 0;
    while {
        _g(GUARD_ID, A.len() as u32 + 1);
        i < A.len()
    } {
        if A[i] != B[i] {
            rollback(b"_g", -1)
        }
        i += 1;
    }

    accept(b"", 0)
}
