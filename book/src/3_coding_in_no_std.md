# Coding in `no_std`

Coding a hook is special, because it gets compiled down to a very restricted WebAssembly.

Therefore, there are certain rules that you need to follow. The overarching rules are as follows:

1. The WebAssembly hook file cannot contain any other exports than `$hook` and `$cbak`.

   Somewhere in your working WebAssembly hook, something similar to these lines of code will exist:

   ```wat
   (func $cbak (type 1) (param i32) (result i64)
   i64.const 0)
   (func $hook (type 1) (param i32) (result i64)
   (local i32 i32 i64 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)

   ...

   (export "cbak" (func 10))
   (export "hook" (func 11))
   ```

   The moment you want to export some other function, the guard checker will complain and will reject your hook.

2. The WebAssembly hook file can only use certain functions from `env` and nothing else. The hook restricts what functions you can use, so the choices are limited to standard library functions like `_g` that are injected in the execution environment.

But practically, what would these mean for you?

## `no_std` environment

This means that you need to code in `no_std` environment. You must have noticed that at the every first line of each hook, we write `#![no_std]`. This is to let the Rust compiler know that we want to code in `no_std` environment only.

Typically, `no_std` environment is used for programs where you cannot expect to have a standard operating system with network, file system, etc. This is the case for hooks as well, because it is run in a very isolated wasm runtime.

Now, below are some specific tips and rules for coding in hooks with all of above things in mind.

### No imports from `std` or other external crates

Since the hook is supposed to run in a `no_std` environment, Rust compiler will complain if you try to import from `std`. For example, you cannot import and use `Vec`:

```rs
#![no_std]
#![no_main]

// import does not work here due to the following error:
// failed to resolve: use of undeclared crate or module `alloc`
// add `extern crate alloc` to use the `alloc` crate
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn cbak(_: u32) -> i64 {
    0
}

#[no_mangle]
pub extern "C" fn hook(_: u32) -> i64 {
    max_iter(1);
    let a = Vec::new();
}
```

Any other imports except from `core` will not work. The reason `core` imports will work is that it only contains the parts that are strictly irrelevant to the operating system. But your choices will be very much limited compared to `std`, still.

You are also not allowed to use crates that are not `no-std` for the same reason.

### All functions must be `inline`

Below hook file will be compiled into wasm, but will get rejected by the guard checker:

```rs
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
    let bar = foo("test");
    0
}

#[no_mangle]
pub fn foo(bar: &str) -> &str {
    bar
}
```

because it will create a wasm file that looks like:

```wat
(export "cbak" (func $cbak))
(export "hook" (func $hook))
(export "foo" (func $foo))
```

And this is a violation of the aforementioned rule, where no other exports other than `$cbak` and `$hook` are alllowed.

This is same for the functions without `#[no_mangle]` too:

```rs
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
    let bar = foo(0);
    0 + bar as i64
}

pub fn foo(bar: u32) -> u32 {
    (bar * 2 * 3 / 4) * mul_iter(bar)
}

pub fn mul_iter(bar: u32) -> u32 {
    let mut ret = 1;

    let mut counter = 0;
    while {
        max_iter(11);
        counter < 10
    } {
        counter += 1;
        ret *= bar;
    }
    ret
}
```

Above hook code might get passed by the guard checker. The reason is that it is up to the Rust compiler to decide if it wants to inline the functions `foo` and `mul_iter`, and to further optimize the call by declaring other utility functions as well. Remember, there are only two functions allowed: `hook` and `cbak`.

But sometimes, the compiler would produce this webassembly file:

```wat
...
(func $cbak (type 1) (param i32) (result i64)
    i64.const 0)
  (func $hook (type 1) (param i32) (result i64)
    (local i32)
    i32.const 0
    i32.const 0
    i32.load offset=1048576
    i32.const 1
    i32.add
    local.tee 1
    i32.store offset=1048576
    local.get 1
    i32.const 1
    call $_g
    drop
    call $_ZN14array_equality3foo17h742873b0c0345c82E
    i64.const 0)
  (func $_ZN14array_equality3foo17h742873b0c0345c82E (type 2)
    call $_ZN14array_equality8mul_iter17h8cf42c9c1cef5607E)
  (func $_ZN14array_equality8mul_iter17h8cf42c9c1cef5607E (type 2)
    (local i32)
    i32.const 0
    i32.const 0
    i32.load offset=1048576
...
```

We can see that the compiler injects `$_ZN14array_equality3foo17h742873b0c0345c82E` even if we didn't instruct it to. All sorts of things can happen when you don't explicitly `inline` the function.

Therefore, the solution is to label all other functions you write with `#[inline(always)]`:

```rs
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
    let bar = foo(0);
    0 + bar as i64
}

#[inline(always)]
pub fn foo(bar: u32) -> u32 {
    (bar * 2 * 3 / 4) * mul_iter(bar)
}

#[inline(always)]
pub fn mul_iter(bar: u32) -> u32 {
    let mut ret = 1;

    let mut counter = 0;
    while {
        max_iter(11);
        counter < 10
    } {
        counter += 1;
        ret *= bar;
    }
    ret
}
```

And this will always create a valid hook.

### No direct array declaration when using pointers

Let's say you will want to interact with the underlying `C` API right away. In that case, you might want to write something like:

```rs
let mut buffer = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,]
unsafe {
  c::hook_account(buffer.as_mut_ptr() as u32, 20 as u32)
}
```

But the guard checker will reject your hook, because the Rust compiler will inject `$memset` function into your hook:

```wat
(func $_ZN17compiler_builtins3mem6memset17he135806270db418bE (type 3) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.const 16
        i32.ge_u
        br_if 0 (;@2;)
        local.get 0
        local.set 3
        br 1 (;@1;)
      end
      local.get 0

...

(func $memset (type 3) (param i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    call $_ZN17compiler_builtins3mem6memset17he135806270db418bE)
```

Instead, you will need to always use `MaybeUninit`. This will prevent any `memset` or `memcpy` functions from appearing. Here's how you can create a working hook out of the same thing:

```rs
let mut uninitialized_buffer: [MaybeUninit<u8>; 20] = MaybeUninit::uninit_array();

let buffer: [u8; 20] = unsafe {
    let result = c::hook_account(uninitialized_buffer.as_mut_ptr() as u32, 20 as u32).into()

    match result {
        Ok(_) => {}
        Err(err) => {
            return Err(err);
        }
    }

    uninitialized_buffer
        .as_ptr()
        .cast::<[u8; BUFFER_LEN]>()
        .read_volatile()
};

// now use buffer
let first = buffer[0]
```

But for normal occassions where you would not do anything special with the pointer to the array, the resulting wasm file looks fine (needs a bit more research though), so you can just use it like:

```rs
// this creates a byte array, but will not be rejected
let hello = b"hello";

// this too
let a = [1,2,3];

let len = hello.len() + a.len();
```

### Fighting for smaller WebAssembly bytesize

You will need to use unsafe pointers. This section is WIP.

### Volatile reads

`read_volatile` should be used when you don't want the compiler to inject some other code for you to optimize under the hood, potentially declaring another function in your code.

An alternative is to use an unsafe pointer, but this comes at the cost of making the code potentially more dangerous.

But unsafe pointer, compared to volatilely-read array, will almost always produce significantly smaller bytesizes. We care about bytesize because this is directly proportional to the fee you need to pay for when you register your hook via `SetHook` transaction and the fee that the people that want to run your hook will pay. This is partciuarly true when you array is big.

Therefore, it is recommended to use unsafe pointer instead of volatile reads if you care the most about the fee.

### Avoiding undefined behavior

Always follow the rules laid out in the Rust documentation when using `unsafe` functions. This section is WIP.
