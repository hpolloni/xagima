#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(default_alloc_error_handler)]


use alloc::boxed::Box;
use bootloader::BootInfo;
use core::panic::PanicInfo;
use xagima::pci;
use xagima::println;

extern crate alloc;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    xagima::halt();
}

trait NetworkDriver {}

struct E1000Driver;

// TODO: extract as trait
impl E1000Driver {
    pub fn new(pci_info: pci::Device) -> Self {
        println!("Initializing e1000 driver at base address {:#x}", pci_info.bar[0]);

        todo!("Continue initialization");
    }

    fn _detect_eeprom(pci: &pci::Device) -> bool {
        const EEPROM_OFFSET: usize = 0x0014;
        pci.write(EEPROM_OFFSET, 0x1 as u32);
        let mut eeprom_found = false;
        let mut iter = 0;
        while !eeprom_found && iter < 1000 {
            if pci.read::<u32>(EEPROM_OFFSET) & 0x10 != 0 {
                eeprom_found = true
            }
            iter += 1;
        }
        return eeprom_found;
    }
}

impl NetworkDriver for E1000Driver {

}

struct Ne2kDriver;

impl Ne2kDriver {
    pub fn new(pci: pci::Device) -> Self {
        println!("Initializing ne2k driver at base address {:#x}", pci.bar[0]);
        // Reset
        // The fact that we panic if we fail to reset means we should really return a result instead.
        pci.write(0x1F, pci.read::<u8>(0x1F));
        let mut reset_done = pci.read::<u8>(0x07) & 0x80;
        let mut iter = 0;
        while reset_done == 0 {
            reset_done = pci.read::<u8>(0x07) & 0x80; 
            if iter > 10000 {
                panic!("Failed to reset device")
            }
            iter += 1;
        }
        pci.write::<u8>(0x07, 0xFF);        // mask interrupts
        pci.write::<u8>(0x00, (1<<5) | 1);  // page 0
        pci.write::<u8>(0x0E, 0x49);        // set access
        pci.write::<u8>(0x0A, 0x00);        // clear count registers
        pci.write::<u8>(0x0B, 0x00);
        pci.write::<u8>(0x0F, 0x00);        // mask completion IRQ
        pci.write::<u8>(0x07, 0xFF);
        pci.write::<u8>(0x0C, 0x20);        // set to monitor
        pci.write::<u8>(0x0D, 0x02);        // and loopback mode.
        pci.write::<u8>(0x0A, 32);          // reading 32 bytes
        pci.write::<u8>(0x0B, 0x00);        // count high
        pci.write::<u8>(0x08, 0x00);        // start DMA at 0
        pci.write::<u8>(0x09, 0x00);        // start DMA high
        pci.write::<u8>(0x00, 0x0A);        // start the read

        let mut prelude: [u8; 32] = [0; 32];

        for i in 0..32 {
            prelude[i] = pci.read::<u8>(0x10)
        }

        println!("MAC Address");
        for i in 0..6 {
            println!("{:#x}", prelude[i]);
        }

        panic!("Continue initialization");
    }
}

impl NetworkDriver for Ne2kDriver {

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
            println!("Found ne2k compatible device. Setting it as main network device.");

            network_device = Some(Box::new(Ne2kDriver::new(device)));
        }
    }

    panic!("No network device found!");
}
