#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print!("Hello world");
    println!(", some numbers: {} {}", 42, 1.337);
    panic!("Kernel panic");
    loop {}
}
