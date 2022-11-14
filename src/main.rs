#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(default_alloc_error_handler)]

use bootloader::BootInfo;
use core::panic::PanicInfo;
use xagima::pci;
use xagima::println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    xagima::halt();
}

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    xagima::init(boot_info);

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

    panic!("End of main");
}
