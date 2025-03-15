//! A pinocchio-based Memo (aka 'p-memo') program.

#![no_std]

mod entrypoint;

pinocchio::no_allocator!();
pinocchio::nostd_panic_handler!();
