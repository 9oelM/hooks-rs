# Creating a basic hook

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
    // Every hook needs to import guard function
    // and use it at least once
    max_iter(1);

    // Log it to the debug stream, which can be inspected on the website or via wss connection
    let _ = trace(b"accept.rs: Called.", b"", DataRepr::AsUTF8);

    // Accept all
    accept(b"accept.rs: Finished.", line!().into());
}
```

Here's the most basic hook you can write. Let's go through one by one.

First of all, let's talk about `#![no_std]` and `#![no_main]` attributes:

`#![no_std]`: we need this because hooks run in a very limited environment where standard libraries or external function calls are not available. This is equivalent to saying we are not going to import anything from `std` in Rust. For example, you cannot write:

```rs
use std::vec::Vec;
```

because `Vec` is from `std`. Otherwise, Rust has no way of knowing that your hook file has no access to `std`.

`#![no_main]`: notice that our hook file doesn't have `main` function. we don't need `main` function because we only intend to export two functions only: `cbak` and `hook`. Any other function exports are all ignored and actually rejected when the hook file is submitted.

Next, we import everything from hooks-rs:

```rs
use hooks_rs::*;
```

This is the only thing that you need to do to be able to access all of the APIs.

After that, we have `#[no_mangle]`. This tells the compiler that we don't want the name of the function to be 'mangle'd, which in turn would create dynamic names and cause an XRPL node to fail to call the function from the generated wasm file correctly, because it only knows about `cbak` and `hook`, not something like `$cbak_125agh4`. This is only needed for `cbak` and `hook` functions. Usage on any other functions will cause them to be regarded as an export in the resulting webassembly file, which will make the SetHook transaction fail to be validated.

Phew, now we have the first function: `cbak`.

```rs
pub extern "C" fn cbak(_: u32) -> i64 {
    0
}
```

The presence of this function is actually optional, which means it can be omitted. `cbak` (short for 'callback') function is called if `hook` function emits a transaction. We will cover `cbak` in greater detail in other chapters, so leave it there for now.

```rs
pub extern "C" fn hook(_: u32) -> i64 {
    // Every hook needs to import guard function
    // and use it at least once
    max_iter(1);
    ...
}
```

Now, we have the `hook` function. The first line inside it cals `max_iter`. One important characteristic about hooks is that it is NOT turing-complete. This means you somehow want to specify how long or how much a program would run. `max_iter` is an easy way to do that. Otherwise, you will have to use `c::_g`, which is a direct call to the external C API. But you will hardly need to use this because every single call to `c::_g` can be replaced by `max_iter`.

```rs
let _ = trace(b"accept.rs: Called.", b"", DataRepr::AsUTF8);
```

The next line is a trace call. It's similar to `console.log` in JavaScript or `cout` in `C++`. It is for debugging purposes. At the time of writing, you can establish a manual websocket connection to `wss://xahau-test.net/{r-address}` or navigate to [https://hooks-testnet-v3-debugstream.xrpl-labs.com/](https://hooks-testnet-v3-debugstream.xrpl-labs.com/) on your browser.

When the hook gets executed, the log should appear on the browser. If you intend to inspect logs from CLI, the easiest way is to install any websocket connection tools liks `websocat` and just run something like `websocat "wss://xahau-test.net/rL36bt3dv4o27hJup1hrKN2XfnzhYUQ5ez"` and the logs will start appearing if there's something happening with `rL36bt3dv4o27hJup1hrKN2XfnzhYUQ5ez`.

Finally, there's an accept call:

```rs
accept(b"accept.rs: Finished.", line!().into());
```

Calling `accept` would accept the transaction, meaning that it will make the incoming transaction validated as long as there are no other problems. The message `accept.rs: Finished.` will be recorded in the ledger as one of the transaction details, so people know what happened with this hook when someone ran it. `line!().into()` is a call to `line!` macro. It's similar to `__LINE__` in C. At compile time, Rust compiler will look at which line this code is populated at, and will replace it with that that line number. In this case, it will be 22. Note that this is primarily for debugging purposes; In production, you will want to use something more meaningful as a hook return code so that you know what happened. This will also be recorded in the ledger.
