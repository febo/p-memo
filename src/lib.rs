//! A pinocchio-based Memo (aka 'p-memo') program.

#![no_std]

mod entrypoint;

// No allocator needed.
pinocchio::no_allocator!();

/// A panic handler for `no_std`.
///
/// TODO: Upstream this to `pinocchio` crate.
#[cfg(not(test))]
#[no_mangle]
#[panic_handler]
fn custom_panic(info: &core::panic::PanicInfo<'_>) -> ! {
    if let Some(location) = info.location() {
        pinocchio::log::sol_log(location.file());
    }
    // Panic reporting.
    pinocchio::log::sol_log("** PANICKED **");
    // Never returns.
    loop {}
}
