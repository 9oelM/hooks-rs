//! XRPL Hooks API
//!
//! This crate allows you to write XRPL hooks in Rust.

#![no_std]
#![no_builtins]
#![deny(
    warnings,
    clippy::all,
    missing_copy_implementations,
    missing_docs,
    rustdoc::missing_crate_level_docs,
    non_ascii_idents,
    unreachable_pub
)]
#![doc(test(attr(deny(warnings))))]
#![feature(maybe_uninit_uninit_array, maybe_uninit_array_assume_init)]

#[allow(missing_docs)]
pub mod c {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

/// XRPL Hooks API
pub mod api;

/// A few utilities
pub mod helpers;

/// Transaction builders
pub mod transaction;

// Prelude
pub use {api::*, helpers::*, transaction::*};

#[cfg(not(test))]
use core::panic::PanicInfo;
/// You should use rollback() instead of native panic!() macro
#[cfg(not(test))]
#[inline(always)]
#[panic_handler]
fn panic(_: &PanicInfo<'_>) -> ! {
    loop {}
}
