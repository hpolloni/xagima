#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod vga;
mod print;

use core::panic::PanicInfo;
use bootloader::BootInfo;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(_: InterruptStackFrame) {
    println!("BREAKPOINT!");
}

#[no_mangle]
pub extern "C" fn _start(_boot_info: &'static BootInfo) -> ! {
    init_idt();
    x86_64::instructions::interrupts::int3();

    panic!("End of message");
}
