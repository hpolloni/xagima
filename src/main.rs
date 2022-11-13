#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_harness::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga;
mod print;
mod pci;
mod interrupts;
mod serial;
mod test_harness;

use core::panic::PanicInfo;
use bootloader::BootInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_harness::panic_handler(info);
}

#[no_mangle]
pub extern "C" fn _start(_boot_info: &'static BootInfo) -> ! {
    interrupts::init();

    // TODO: gdt / tss
    // TODO: paging
    // PCI scan
    for bus in 0..255 {
        for slot in 0..32 {
            for function in 0..8 {
                let pci_device = pci::probe(pci::Location::new(bus, slot, function));
                if pci_device.is_some() {
                    let device = pci_device.unwrap();
                    println!("Found device: {:?}", device);
                }
            }
        }
    }
 
    #[cfg(test)]
    test_main();
    
    panic!("End of main");
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}