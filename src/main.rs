#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod vga;
mod print;
mod pci;

use core::panic::PanicInfo;
use bootloader::BootInfo;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/* TODO: move to its own module */
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

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("BREAKPOINT");
    println!("Stack frame: {:#?}", stack_frame);
}

#[no_mangle]
pub extern "C" fn _start(_boot_info: &'static BootInfo) -> ! {
    init_idt();

    // TODO: gdt / tss
    // TODO: paging
    // PCI scan diagnostics
    for bus in 0..255 {
        for slot in 0..32 {
            for function in 0..8 {
                let pci_device = pci::probe(pci::Location::new(bus, slot, function));
                if pci_device.is_some() {
                    let device = pci_device.unwrap();
                    println!("Found device: {:#x} {:#x} at {:?}", device.vendor_id, device.device_id, device.location);
                }
            }
        }
    }
    panic!("End of main");
}
