use crate::{pci, println};

use super::{MacAddress, NetworkDriver};

pub struct E1000Driver;

const EEPROM_OFFSET: usize = 0x0014;

impl E1000Driver {
    pub fn new(pci_info: pci::Device) -> Self {
        println!(
            "Initializing e1000 driver at base address {:#x}",
            pci_info.bar[0]
        );
        let eeprom_exists = Self::detect_eeprom(&pci_info);
        if !eeprom_exists {
            todo!("Unsupported configuration: No EEPROM");
        }

        let mut mac_addr: [u8; 6] = [0; 6];

        [mac_addr[0], mac_addr[1]] = Self::eeprom_read(&pci_info, 0).to_le_bytes();
        [mac_addr[2], mac_addr[3]] = Self::eeprom_read(&pci_info, 1).to_le_bytes();
        [mac_addr[4], mac_addr[5]] = Self::eeprom_read(&pci_info, 2).to_le_bytes();

        let mac = MacAddress::new(mac_addr);
        println!("MAC Address: {}", mac);
        todo!("E1000 driver WIP");
    }

    fn detect_eeprom(pci: &pci::Device) -> bool {
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

    fn eeprom_read(pci: &pci::Device, offset: u8) -> u16 {
        pci.write(EEPROM_OFFSET, (1) | (offset as u32) << 8);
        let mut tmp = pci.read::<u32>(EEPROM_OFFSET);
        while tmp & (1 << 4) == 0 {
            tmp = pci.read::<u32>(EEPROM_OFFSET);
        }
        return ((tmp >> 16) & 0xFFFF) as u16;
    }
}

impl NetworkDriver for E1000Driver {}
