//! A pinocchio-based Memo (aka 'p-memo') program.

#![no_std]

mod entrypoint;

pinocchio::no_allocator!();
#[cfg(not(feature = "logging"))]
pinocchio::nostd_panic_handler!();
