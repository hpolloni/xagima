#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_harness::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod vga;
pub mod print;
pub mod pci;
pub mod interrupts;
pub mod serial;
pub mod test_harness;

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    test_harness::panic_handler(info)
}
