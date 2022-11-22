#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(default_alloc_error_handler)]

use alloc::boxed::Box;
use bootloader::BootInfo;
use core::panic::PanicInfo;
use xagima::net::e1000::E1000Driver;
use xagima::net::ne2k_pci::Ne2kPciDriver;
use xagima::net::NetworkDriver;
use xagima::pci;
use xagima::println;

extern crate alloc;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    xagima::halt();
}

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    xagima::init(boot_info).unwrap();

    // TODO: Discovery should be encapsulated in a device manager abstraction
    // TODO: Support multiple network cards.
    let pci_devices = pci::scan();

    let mut network_device: Option<Box<dyn NetworkDriver>> = Option::None;
    for device in pci_devices {
        if device.vendor_id == 0x8086 && device.device_id == 0x100e {
            println!("Found e1000 network card. Setting it as main network device.");
            network_device = Some(Box::new(E1000Driver::new(device)));
            break;
        }
        if device.vendor_id == 0x10ec && device.device_id == 0x8029 {
            println!("Found ne2k PCI compatible device. Setting it as main network device.");

            network_device = Some(Box::new(Ne2kPciDriver::new(device)));
        }
    }

    panic!("No network device found!");
}
