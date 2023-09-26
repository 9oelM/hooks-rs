# hooks-rs

XRPL hooks in Rust.

> [!WARNING]
> The repository is not ready yet. You will need to use it at your own discretion.

## Prerequisites

- Install Rust nightly.
- Build and make [`hook-cleaner`](https://github.com/XRPLF/hook-cleaner-c) available in your `PATH`.
- Install [`wasm-opt` from binaryen repo](https://github.com/WebAssembly/binaryen/releases) and make it available in your `PATH`.

## Examples

See [examples](./examples/).

## Documentation (WIP)

The book covers very few things for now but you can have a look: https://9oelm.github.io/hooks-rs/

## Supported functions

Control

- [x] `accept`
- [x] `rollback`

Utilities

- [ ] `util_raddr`
- [ ] `util_accid`
- [ ] `util_verify`
- [ ] `util_sha512h`
- [ ] `util_keylet`

Serialization

- [ ] `sto_subfield`
- [ ] `sto_subarray`
- [ ] `sto_emplace`
- [ ] `sto_erase`
- [ ] `sto_validate`

Emitted transaction

- [ ] `etxn_burden`
- [x] `etxn_details`
- [x] `etxn_fee_base`
- [x] `etxn_nonce`
- [x] `etxn_reserve`
- [ ] `etxn_generation`
- [x] `emit`

Float

- [x] `float_set`
- [x] `float_multiply`
- [x] `float_mulratio`
- [x] `float_negate`
- [x] `float_compare`
- [x] `float_sum`
- [ ] `float_sto`
- [x] `float_sto_set`
- [x] `float_invert`
- [x] `float_divide`
- [x] `float_one`
- [x] `float_exponent`
- [x] `float_mantissa`
- [x] `float_sign`
- [x] `float_int`
- [ ] `float_root`
- [ ] `float_log`

Ledger

- [ ] `fee_base`
- [x] `ledger_seq`
- [ ] `ledger_last_hash`
- [ ] `ledger_last_time`
- [ ] `ledger_nonce`
- [ ] `ledger_keylet`

Hook context

- [x] `hook_account`
- [ ] `hook_hash`
- [x] `hook_param`
- [ ] `hook_param_set`
- [ ] `hook_skip`
- [ ] `hook_pos`
- [ ] `hook_again`

Slot

- [ ] `slot`
- [ ] `slot_clear`
- [ ] `slot_count`
- [ ] `slot_set`
- [ ] `slot_size`
- [ ] `slot_subarray`
- [ ] `slot_subfield`
- [ ] `slot_type`
- [ ] `xpop_slot`
- [ ] `slot_float`

State

- [x] `state`
- [x] `state_set`
- [ ] `state_foreign`
- [ ] `state_foreign_set`

Trace

- [x] `trace`
- [x] `trace_num`
- [x] `trace_float`

Originating transaction

- [ ] `otxn_burden`
- [x] `otxn_field`
- [ ] `otxn_generation`
- [ ] `otxn_id`
- [ ] `otxn_type`
- [ ] `otxn_slot`
- [x] `otxn_param`
- [ ] `meta_slot`

## C bindings

The latest header files can be found at https://github.com/XRPLF/hook-macros

## Credits

- This is a fork of [otov4its/xrpl-hooks](https://github.com/otov4its/xrpl-hooks) which has been unmaintained for a long time.
- Lots of examples and codes have been adopted from [dangell7](https://github.com/dangell7)'s repositories.

## Contributing

There are still many parts of the C API that are not supported yet. The first priority for now would be to get all of them working.

If you have all checkboxes below ticked, you can proceed to create a PR:

- [ ] You have implemented something that wasn't here before
- [ ] You have created an integration test to prove that it works. See `/tests` folder to see the existing integration test examples. Currently, each integration test would cover the smallest unit of testable 'chunk', because hooks is impossible to be tested without a live connection with an actual XRPL node, no matter if it is on local docker or is one of the testnet nodes. Therefore, an integration test is somewhat like an unit test but still connects to an actual XRPL node to run the hook to see if it actually works. Make sure the all of the state changes caused by the hook are correctly asserted, before and after the execution of the hook.
- [ ] You have created an example in `/examples` folder. The example hook file should contain the most minimal example of how the feature that you have added can be used. Use this example as a documentation for the function as well, so that it can appear in the generated documentation.
- [ ] CI is passing. Unless all workflows give you a green tick, the PR is unlikely to be reviewed.
- [ ] The PR has a sufficient description of what it adds or changes. The title of the PR would usually just follow semantic commit messages convention. Do not worry about the commit messages themselves that much, since they are going to be squashed anyway.

Remember that this repository is a novel combination of two amazing concepts: Rust and Hooks. And neither of these are easy nor familiar with the general audience. If you are not sure where to start, probably start with the book, which will give you some idea to start with.
