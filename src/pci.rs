use core::fmt::Display;

const CONFIG_ADDRESS: u16 = 0xCF8;
const CONFIG_DATA: u16 = 0xCFC;

#[derive(Copy, Clone, Debug)]
pub struct Location {
    bus: u8,
    slot: u8,
    func: u8,
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
    pub bar: [u32; 6],
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

fn config_read(location: Location, offset: u8) -> u16 {
    let bus32: u32 = location.bus.into();
    let slot32: u32 = location.slot.into();
    let func32: u32 = location.func.into();
    let offset32: u32 = offset.into();

    let mask = 0x80000000u32;
    let address = (bus32 << 16) | (slot32 << 11) | (func32 << 8) | (offset32 & 0xfc) | mask;

    let mut config_address = x86_64::instructions::port::PortWriteOnly::new(CONFIG_ADDRESS);
    let mut config_data = x86_64::instructions::port::PortReadOnly::new(CONFIG_DATA);
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

pub fn probe(location: Location) -> Option<Device> {
    let vendor_id = config_read(location, 0x00);
    if vendor_id == 0xFFFF {
        return None;
    }
    let device_id = config_read(location, 0x02);

    let [prog_if, revision] = config_read(location, 0x08).to_be_bytes();
    let [class_id, subclass_id] = config_read(location, 0x0A).to_be_bytes();
    let [_, header_type] = config_read(location, 0x0E).to_be_bytes();

    let mut bar: [u32; 6] = [0; 6];
    if header_type == 0x00 {
        for i in 0..6 {
            bar[i] = config_read32(location, 0x10 + (i as u8) * 4);
        }
    }

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
    })
}
