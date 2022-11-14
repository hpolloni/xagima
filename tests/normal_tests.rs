#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(xagima::testing::runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(default_alloc_error_handler)]

extern crate alloc;

use alloc::boxed::Box;

use bootloader::BootInfo;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    xagima::testing::default_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    xagima::init(boot_info);
    test_main();
    xagima::halt();
}

#[test_case]
fn trivial_test() {
    assert_eq!(1, 1);
}

#[test_case]
fn can_create_boxes() {
    let answer = Box::new(42);

    assert_eq!(*answer, 42);
}
