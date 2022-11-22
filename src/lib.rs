#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(default_alloc_error_handler)]

use bootloader::BootInfo;
use memory::MemoryError;

pub mod gdt;
pub mod heap;
pub mod interrupts;
pub mod memory;
pub mod net;
pub mod pci;
pub mod print;
pub mod serial;
pub mod testing;
pub mod vga;

extern crate alloc;

#[derive(Debug, Clone, Copy)]
pub enum OsError {
    MemoryError { err: MemoryError },
}

impl From<MemoryError> for OsError {
    fn from(err: MemoryError) -> Self {
        Self::MemoryError { err }
    }
}
pub fn init(boot_info: &'static BootInfo) -> Result<(), OsError> {
    gdt::init();
    interrupts::init();
    memory::init(boot_info)?;
    Ok(())
}

pub fn halt() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
