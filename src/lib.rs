#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

pub mod testing;
pub mod print;
pub mod vga;
pub mod serial;
pub mod interrupts;
pub mod pci;

pub fn init() {
    interrupts::init();
    // TODO: gdt / tss
    // TODO: paging
}