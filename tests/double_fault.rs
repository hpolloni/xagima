#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(xagima::testing::runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::BootInfo;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    xagima::testing::success();
    loop{}
}

#[no_mangle]
pub extern "C" fn _start(_boot_info: &'static BootInfo) -> ! {
    test_main();
    xagima::testing::fail();
    loop {}
}

#[test_case]
fn test() {
    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    }
}