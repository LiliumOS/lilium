#![cfg_attr(not(test), no_std)]
#![feature(naked_functions, thread_local, lazy_cell)]

extern crate alloc;

mod tls_impl;

pub mod default_alloc;

#[doc(hidden)]
pub mod detect;
pub mod handle;
pub mod rand;
