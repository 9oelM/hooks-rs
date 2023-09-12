# hooks-rs

XRPL hooks in Rust.

> [!WARNING]
> The repository is not ready yet. You will need to use it at your own discretion.

## Prerequisites

- Install Rust nightly.
- Build and make [`hook-cleaner`](https://github.com/XRPLF/hook-cleaner-c) available in your `PATH`.

## Examples

See [examples](./examples/).

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
- [x] `float_sto`
- [ ] `float_sto_set`
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
