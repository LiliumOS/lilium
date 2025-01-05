#![cfg_attr(not(test), no_std)]
#![feature(naked_functions, thread_local)]

extern crate alloc;

mod tls_impl;

mod low_level;

pub mod default_alloc;

#[doc(hidden)]
pub mod detect;
pub mod handle;
pub mod io;
pub mod rand;

pub use low_level::halt;
