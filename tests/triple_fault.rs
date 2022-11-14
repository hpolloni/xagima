#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(xagima::testing::runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(default_alloc_error_handler)]

use bootloader::BootInfo;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    xagima::testing::success();
    xagima::halt();
}

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    xagima::init(boot_info);
    test_main();
    xagima::testing::fail();
    xagima::halt();
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    let mut value = 0u32;
    volatile::Volatile::new(&mut value).read(); // prevent tail recursion optimizations
}

#[test_case]
fn test() {
    stack_overflow();
}
