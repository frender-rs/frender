#![no_std]
// This crate is only expected to be used in const compile time
// so safety and correctness are chosen over performance.
#![forbid(unsafe_code)]

pub mod slice;
pub mod str;

pub mod convert;
