//! Peripheral access API for STM32L1 microcontrollers
//! (generated using [svd2rust])
//! [svd2rust]: https://github.com/japaric/svd2rust
//!
//! You can find an overview of the API here:
//! https://docs.rs/svd2rust/0.8.1/svd2rust/#peripheral-api
//!
//! For more details see the README here:
//! https://github.com/adamgreig/stm32-rs

#![allow(non_camel_case_types)]
#![allow(private_no_mangle_statics)]
#![feature(const_fn)]
#![feature(optin_builtin_traits)]
#![no_std]

#![cfg_attr(feature = "rt", feature(global_asm))]
#![cfg_attr(feature = "rt", feature(use_extern_macros))]
#![cfg_attr(feature = "rt", feature(used))]

extern crate vcell;
extern crate bare_metal;
extern crate cortex_m;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::{default_handler, exception};

#[cfg(feature = "stm32l100")]
pub mod stm32l100;

#[cfg(feature = "stm32l151")]
pub mod stm32l151;

#[cfg(feature = "stm32l162")]
pub mod stm32l162;

