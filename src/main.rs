#![no_std]
#![no_main]

mod vga;
mod print;

use core::panic::PanicInfo;
use bootloader::BootInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start(_boot_info: &'static BootInfo) -> ! {
    panic!("End of message");

    loop {}
}
