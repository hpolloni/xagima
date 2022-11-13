
const CONFIG_ADDRESS: u16 = 0xCF8;
const CONFIG_DATA: u16 = 0xCFC;

#[derive(Copy, Clone, Debug)]
pub struct Location {
    bus: u8,
    slot: u8,
    func: u8
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
        return config_data.read();
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
        return None
    }
    let device_id = config_read(location, 0x02);
    let revision = (config_read(location, 0x08) & 0x00FF) as u8;
    let prog_if = (config_read(location, 0x08) & 0xFF00) as u8;
    let subclass_id = (config_read(location, 0x0A) & 0x00FF) as u8;
    let class_id = ((config_read(location, 0x0A) & 0xFF00) >> 8) as u8;
    let header_type = (config_read(location, 0x0E) & 0x00FF) as u8;
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
        bar
    })
}

/*

      // TODO: other header types
      if (header_type == 0x00) {
        for (int i = 0; i < 6; i++) {
          bar[i] = config_read32(bus, slot, function, 0x10 + i * 4);
        }
      }
    }

    struct pci_header {
    uint8_t bus, slot, function;



    uint32_t bar[6];
*/
