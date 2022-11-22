use alloc::vec::Vec;
use core::fmt::Display;
use voladdress::{Unsafe, VolAddress};
use x86_64::{
    instructions::port::{PortReadOnly, PortWriteOnly},
    structures::port::{PortRead, PortWrite},
};

const CONFIG_ADDRESS: u16 = 0xCF8;
const CONFIG_DATA: u16 = 0xCFC;

#[derive(Copy, Clone, Debug)]
pub struct Location {
    pub bus: u8,
    pub slot: u8,
    pub func: u8,
}

impl Location {
    pub fn new(bus: u8, slot: u8, func: u8) -> Self {
        Self { bus, slot, func }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Device {
    pub location: Location,
    pub vendor_id: u16,
    pub device_id: u16,
    pub revision: u8,
    pub prog_if: u8,
    pub subclass_id: u8,
    pub class_id: u8,
    pub header_type: u8,
    pub bar: [usize; 6],
    pub irq: u8,
}

impl Display for Device {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "PCIDevice {{")?;
        writeln!(f, " vendor_id: {:#x}", self.vendor_id)?;
        writeln!(f, " device_id: {:#x}", self.device_id)?;
        writeln!(f, " class_id: {:#x}", self.class_id)?;
        writeln!(f, " subclass_id: {:#x}", self.subclass_id)?;
        writeln!(f, " ioaddr: {:#x}", self.bar[0])?;
        writeln!(f, " }}")
    }
}

impl Device {
    // TODO: I feel these should be per BAR and not at the device level
    // So the API should be something like pci.bar[0].read()...however,
    // pci.read may still default to bar[0]
    pub fn write<T: Copy + PortWrite>(&self, offset: usize, value: T) {
        if self.bar[0] & 1 == 0 {
            // Memory Space BAR
            if (self.bar[0] >> 1) & 3 == 0 {
                // 32-bit address
                let mem_base: usize = self.bar[0] & 0xFFFFFFF0 as usize;
                let addr = mem_base + offset;
                unsafe {
                    let ptr: VolAddress<T, (), Unsafe> = VolAddress::new(addr);
                    ptr.write(value);
                }
            } else if (self.bar[0] >> 1) & 3 == 2 {
                todo!("unsupported 64-bit base address");
            } else {
                panic!("Unknown base address register type");
            }
        } else {
            // I/O Space BAR
            let io_base = self.bar[0] & !0x3;
            let addr = io_base + offset;
            let mut port = PortWriteOnly::new(addr as u16);
            unsafe {
                port.write(value);
            }
        }
    }

    pub fn read<T: Copy + PortRead>(&self, offset: usize) -> T {
        if self.bar[0] & 1 == 0 {
            // Memory Mapped I/O
            if (self.bar[0] >> 1) & 3 == 0 {
                let mem_base: usize = self.bar[0] & 0xFFFFFFF0 as usize;
                let addr = mem_base + offset;
                unsafe {
                    let ptr: VolAddress<T, Unsafe, ()> = VolAddress::new(addr);
                    return ptr.read();
                }
            } else if (self.bar[0] >> 1) & 3 == 2 {
                todo!("unsupported 64-bit base address");
            } else {
                panic!("Unknown base address register type");
            }
        } else {
            // I/O Space BAR
            let io_base = self.bar[0] & !0x3;
            let addr = io_base + offset;
            let mut port = PortReadOnly::new(addr as u16);
            unsafe {
                return port.read();
            }
        }
    }
}

fn config_read(location: Location, offset: u8) -> u16 {
    let bus32: u32 = location.bus.into();
    let slot32: u32 = location.slot.into();
    let func32: u32 = location.func.into();
    let offset32: u32 = offset.into();

    let mask = 0x80000000u32;
    let address = (bus32 << 16) | (slot32 << 11) | (func32 << 8) | (offset32 & 0xfc) | mask;

    let mut config_address = PortWriteOnly::new(CONFIG_ADDRESS);
    let mut config_data = PortReadOnly::new(CONFIG_DATA);
    unsafe {
        /* TODO: lock? */
        config_address.write(address);
        let data: u32 = config_data.read();
        return ((data >> ((offset & 2) * 8)) & 0xFFFF) as u16;
    }
}

fn config_read32(location: Location, offset: u8) -> u32 {
    let lo = config_read(location, offset) as u32;
    let hi = config_read(location, offset + 2) as u32;
    return hi << 16 | lo;
}

pub fn scan() -> Vec<Device> {
    let mut devices = Vec::new();

    for bus in 0..255 {
        for slot in 0..32 {
            for function in 0..8 {
                let pci_device = probe(Location::new(bus, slot, function));
                if pci_device.is_some() {
                    let device = pci_device.unwrap();
                    devices.push(device);
                }
            }
        }
    }
    return devices;
}

pub fn probe(location: Location) -> Option<Device> {
    let vendor_id = config_read(location, 0x00);
    if vendor_id == 0xFFFF {
        return None;
    }
    let device_id = config_read(location, 0x02);

    let [prog_if, revision] = config_read(location, 0x08).to_be_bytes();
    let [class_id, subclass_id] = config_read(location, 0x0A).to_be_bytes();
    let [_, header_type] = config_read(location, 0x0E).to_be_bytes();

    let mut bar: [usize; 6] = [0; 6];
    if header_type == 0x00 {
        for i in 0..6 {
            bar[i] = config_read32(location, 0x10 + (i as u8) * 4) as usize; // TODO: this works in 32bit...will it work in 64?
        }
    }

    let [_, irq] = config_read(location, 0x3C).to_be_bytes();
    Some(Device {
        location,
        vendor_id,
        device_id,
        revision,
        prog_if,
        subclass_id,
        class_id,
        header_type,
        bar,
        irq,
    })
}
