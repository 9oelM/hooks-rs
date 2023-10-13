# Helper APIs

<!-- toc -->

This page covers the helper APIs that are peculiar to writing hooks.

## Loops and `max_iter`

Because hooks are not turing complete, you must let the guard checker know your maximum iteration count for the loops you create. For that reason, only `while` loop is allowed for now.

When you have a loop, you can use `max_iter` to specify that:

```rs
let mut i = 0;
while {
    max_iter(11); // +1 more than the actual iteration count
    i < 10
} {
    // do sth
    i += 1;
}
```

In contrast, this will get rejected by the guard checker:

```rs
let mut i = 0;
while {
    i < 10
} {
    // do sth
    i += 1;
}
```

`max_iter` will automatically inject `c::_g` under the hood, which is the actual c `extern` function for guarding the loop.

Currently, the only way to create a loop is with `while` keyword. `for`-loop like syntax is WIP as it will require a custom macro.

## Array comparison

Comparing arrays is a typical operation. But as we just discussed, `max_iter` imposes a restriction for you to always use a while loop to compare it like this:

```rs
const A: &[u8; 14] = b"same same same";
const B: &[u8; 14] = b"same same diff";

let mut i = 0;
while {
    max_iter(A.len() as u32 + 1); // 14
    i < A.len()
} {
    if A[i] != B[i] {
        rollback(b"diff", -1)
    }
    i += 1;
}
```

But we know that this is time consuming and not really semantic. `hooks-rs` standard library offers `ComparableArray` as a solution. You can write:

```rs
const A: &[u8; 14] = b"same same same";
const B: &[u8; 14] = b"same same same";

const COMPARABLE_A: ComparableArray<u8, 14> = ComparableArray::new(A);
const COMPARABLE_B: ComparableArray<u8, 14> = ComparableArray::new(B);

if COMPARABLE_A != COMPARABLE_B {
  rollback(b"diff", -1)
}
```

For a more detailed example, check out [examples/array_equality.rs](https://github.com/9oelM/hooks-rs/blob/main/examples/array_equality.rs).

## Float computation

In hooks, all float values typically need to stay as [XFL](https://github.com/XRPLF/XRPL-Standards/discussions/39), which is a format that is specifically designed for XRPL balances.

In order to use it, just use `XFL` in your hook, like:

```rs
let one_over_two = XFL::one().mulratio(false, 1, 2).unwrap();
let one_over_four = XFL::one().mulratio(false, 1, 4).unwrap();

let one_over_eight = (one_over_two * one_over_four).unwrap();
```

Note that each computation will return `Result<XFL>` type, so you will need to handle errors for that.

For a more detailed example, check out [examples/float.rs](https://github.com/9oelM/hooks-rs/blob/main/examples/float.rs).
