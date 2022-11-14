#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(default_alloc_error_handler)]

use bootloader::BootInfo;

pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod pci;
pub mod print;
pub mod serial;
pub mod testing;
pub mod vga;
pub mod heap;

extern crate alloc;

pub fn init(boot_info: &'static BootInfo) {
    gdt::init();
    interrupts::init();
    memory::init(boot_info);
}

pub fn halt() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
