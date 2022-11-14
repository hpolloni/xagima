#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

pub mod testing;
pub mod print;
pub mod vga;
pub mod serial;
pub mod interrupts;
pub mod pci;
pub mod gdt;

pub fn init() {
    gdt::init();
    interrupts::init();
    // TODO: paging
}

pub fn halt() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}