#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(xagima::testing::runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::BootInfo;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    xagima::testing::default_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start(_boot_info: &'static BootInfo) -> ! {
    test_main();
    loop {}
}

#[test_case]
fn test() {
    x86_64::instructions::interrupts::int3(); // This should not panic.
}