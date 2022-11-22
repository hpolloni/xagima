use alloc::vec;

use crate::{pci, println};

use super::{MacAddress, NetworkDriver};

// Page 0 registers
const REG_RW_COMMAND: usize = 0x00;
const BIT_COMMAND_STOP: u8 = 0b1 << 0;
const BIT_COMMAND_START: u8 = 0b1 << 1;
const BIT_COMMAND_TXP: u8 = 0b1 << 2;
const BIT_COMMAND_DMA_READ: u8 = 0b001 << 3;
const BIT_COMMAND_DMA_WRITE: u8 = 0b010 << 3;
const BIT_COMMAND_DMA_SEND: u8 = 0b011 << 3;
const BIT_COMMAND_DMA_ABORT: u8 = 0b100 << 3;
const BIT_COMMAND_DMA_FIELD: u8 = 0b111 << 3;
const BIT_COMMAND_PAGE1: u8 = 0b01 << 6;
const BIT_COMMAND_PAGE2: u8 = 0b10 << 6;
const BIT_COMMAND_PAGE_FIELD: u8 = 0b11 << 6;

const REG_WR_PAGESTART: usize = 0x01;
const REG_WR_PAGESTOP: usize = 0x02;
const REG_RW_BOUNDARY: usize = 0x03;
const REG_RD_TRANSMITSTATUS: usize = 0x04;
const REG_WR_TRANSMITPAGE: usize = 0x04;
const REG_RD_NCR: usize = 0x05;
const REG_WR_TRANSMITBYTECOUNT0: usize = 0x05;
const REG_WR_TRANSMITBYTECOUNT1: usize = 0x06;
const REG_RW_INTERRUPTSTATUS: usize = 0x07;
const REG_RD_CRDMA0: usize = 0x08;
const REG_WR_REMOTESTARTADDRESS0: usize = 0x08;
const REG_RD_CRDMA1: usize = 0x09;
const REG_WR_REMOTESTARTADDRESS1: usize = 0x09;
const REG_WR_REMOTEBYTECOUNT0: usize = 0x0a;
const REG_WR_REMOTEBYTECOUNT1: usize = 0x0b;

const REG_RD_RECEIVESTATUS: u8 = 0x0c;
const BIT_RECEIVESTATUS_PRX: u8 = (0b1 << 0);
const BIT_RECEIVESTATUS_CRC: u8 = (0b1 << 1);
const BIT_RECEIVESTATUS_FAE: u8 = (0b1 << 2);
const BIT_RECEIVESTATUS_FO: u8 = (0b1 << 3);
const BIT_RECEIVESTATUS_MPA: u8 = (0b1 << 4);

const REG_WR_RECEIVECONFIGURATION: usize = 0x0c;
const BIT_RECEIVECONFIGURATION_SEP: u8 = (0b1 << 0);
const BIT_RECEIVECONFIGURATION_AR: u8 = (0b1 << 1);
const BIT_RECEIVECONFIGURATION_AB: u8 = (0b1 << 2);
const BIT_RECEIVECONFIGURATION_AM: u8 = (0b1 << 3);
const BIT_RECEIVECONFIGURATION_PRO: u8 = (0b1 << 4);
const BIT_RECEIVECONFIGURATION_MON: u8 = (0b1 << 5);

const REG_RD_FAE_TALLY: u8 = 0x0d;

const REG_WR_TRANSMITCONFIGURATION: usize = 0x0d;
const BIT_WR_TRANSMITCONFIGURATION_LOOPBACK: u8 = (0b10 << 0);

const REG_RD_CRC_TALLY: u8 = 0x0e;

const REG_WR_DATACONFIGURATION: usize = 0x0e;
const BIT_DATACONFIGURATION_WTS: u8 = (0b1 << 0);
const BIT_DATACONFIGURATION_BOS: u8 = (0b1 << 1);
const BIT_DATACONFIGURATION_LS: u8 = (0b1 << 2);
const BIT_DATACONFIGURATION_FIFO_8B: u8 = (0b10 << 5);

const REG_RD_MISS_PKT_TALLY: u8 = 0x0f;

const REG_WR_INTERRUPTMASK: usize = 0x0f;
const BIT_INTERRUPTMASK_PRX: u8 = (0b1 << 0);
const BIT_INTERRUPTMASK_PTX: u8 = (0b1 << 1);
const BIT_INTERRUPTMASK_RXE: u8 = (0b1 << 2);
const BIT_INTERRUPTMASK_TXE: u8 = (0b1 << 3);
const BIT_INTERRUPTMASK_OVW: u8 = (0b1 << 4);
const BIT_INTERRUPTMASK_CNT: u8 = (0b1 << 5);
const BIT_INTERRUPTMASK_RDC: u8 = (0b1 << 6);
const BIT_INTERRUPTMASK_RST: u8 = (0b1 << 7);

const REG_RW_IOPORT: usize = 0x10;

// Page 1 registers
const REG_RW_PHYSICALADDRESS0: usize = 0x01;
const REG_RW_CURRENT: usize = 0x07;

const NE2K_PAGE_SIZE: usize = 256;

const NE2K_RAM_BEGIN: usize = 16384;
const NE2K_RAM_END: usize = 32768;
const NE2K_RAM_SIZE: usize = NE2K_RAM_END - NE2K_RAM_BEGIN;

const NE2K_RAM_SEND_BEGIN: usize = 16384;
const NE2K_RAM_SEND_END: usize = 16384 + 6 * NE2K_PAGE_SIZE;
const NE2K_RAM_SEND_SIZE: usize = NE2K_RAM_SEND_END - NE2K_RAM_SEND_BEGIN;

const NE2K_RAM_RECV_BEGIN: usize = NE2K_RAM_SEND_END;
const NE2K_RAM_RECV_END: usize = NE2K_RAM_END;
const NE2K_RAM_RECV_SIZE: usize = NE2K_RAM_RECV_END - NE2K_RAM_RECV_BEGIN;

pub struct Ne2kPciDriver {
    pci: pci::Device,
}

impl Ne2kPciDriver {
    pub fn new(pci: pci::Device) -> Self {
        println!("Initializing ne2k driver at base address {:#x}", pci.bar[0]);
        // Reset
        let this = Self { pci };
        this.reset();
        let mac = this.read_mac_address();
        this.set_mac_address(&mac);

        this
    }

    fn reset(&self) {
        let interrupt_mask = BIT_INTERRUPTMASK_PRX
            | BIT_INTERRUPTMASK_PTX
            | BIT_INTERRUPTMASK_RXE
            | BIT_INTERRUPTMASK_TXE
            | BIT_INTERRUPTMASK_OVW
            | BIT_INTERRUPTMASK_CNT;

        self.pci
            .write::<u8>(REG_RW_COMMAND, BIT_COMMAND_DMA_ABORT | BIT_COMMAND_STOP);
        self.pci.write::<u8>(
            REG_WR_DATACONFIGURATION,
            BIT_DATACONFIGURATION_FIFO_8B | BIT_DATACONFIGURATION_WTS,
        );

        self.pci.write::<u8>(REG_WR_REMOTEBYTECOUNT0, 0x00);
        self.pci.write::<u8>(REG_WR_REMOTEBYTECOUNT1, 0x00);
        self.pci.write::<u8>(
            REG_WR_RECEIVECONFIGURATION,
            BIT_RECEIVECONFIGURATION_AB | BIT_RECEIVECONFIGURATION_AR,
        );
        self.pci.write::<u8>(
            REG_WR_TRANSMITCONFIGURATION,
            BIT_WR_TRANSMITCONFIGURATION_LOOPBACK,
        );

        self.pci
            .write::<u8>(REG_WR_PAGESTART, (NE2K_RAM_RECV_BEGIN >> 8) as u8);
        self.pci
            .write::<u8>(REG_RW_BOUNDARY, (NE2K_RAM_RECV_BEGIN >> 8) as u8);
        self.pci
            .write::<u8>(REG_WR_PAGESTOP, (NE2K_RAM_RECV_END >> 8) as u8);
        self.pci.write::<u8>(REG_RW_INTERRUPTSTATUS, 0xff);
        self.pci.write::<u8>(REG_WR_INTERRUPTMASK, interrupt_mask);
        /*

        m_ring_read_ptr = NE2K_RAM_RECV_BEGIN >> 8;

        */
    }

    fn set_mac_address(&self, mac: &MacAddress) {
        println!("New MAC Address: {}", mac);
        self.pci.write::<u8>(REG_RW_COMMAND, BIT_COMMAND_PAGE1 | BIT_COMMAND_DMA_ABORT | BIT_COMMAND_STOP);
        for i in 0..6 {
            self.pci.write::<u8>(REG_RW_PHYSICALADDRESS0 + i, mac.addr[i]);
        }
        self.pci.write::<u8>(REG_RW_CURRENT, (NE2K_RAM_RECV_BEGIN >> 8) as u8);

        self.pci.write::<u8>(REG_RW_COMMAND, BIT_COMMAND_DMA_ABORT | BIT_COMMAND_START);
        self.pci.write::<u8>(REG_WR_TRANSMITCONFIGURATION, 0xe0);
    }

    fn read_mac_address(&self) -> MacAddress {
        let prom = self.dma_read(0, 6);
        let mac = MacAddress::new(prom[0..6].try_into().unwrap());
        return mac;
    }

    fn dma_read(&self, addr: usize, maxlen: usize) -> alloc::vec::Vec<u8> {
        let command = self.pci.read::<u8>(REG_RW_COMMAND) & !(BIT_COMMAND_PAGE_FIELD | BIT_COMMAND_DMA_FIELD);
        self.pci.write::<u8>(REG_RW_COMMAND, command | BIT_COMMAND_DMA_ABORT);
        self.pci.write::<u8>(REG_RW_INTERRUPTSTATUS, BIT_INTERRUPTMASK_RDC);

        self.pci.write::<u8>(REG_WR_REMOTEBYTECOUNT0, (maxlen & 0xFF) as u8);
        self.pci.write::<u8>(REG_WR_REMOTEBYTECOUNT1,  (maxlen >> 8) as u8);

        self.pci.write::<u8>(REG_WR_REMOTESTARTADDRESS0, (addr & 0xFF) as u8);
        self.pci.write::<u8>(REG_WR_REMOTESTARTADDRESS1, (addr >> 8) as u8);

        let command = self.pci.read::<u8>(REG_RW_COMMAND) & !(BIT_COMMAND_DMA_FIELD);
        self.pci.write::<u8>(REG_RW_COMMAND, command | BIT_COMMAND_DMA_READ);

        let mut result = vec![];
        for _ in 0..maxlen {
            let data = self.pci.read::<u8>(REG_RW_IOPORT);
            result.push(data);
        }

        while self.pci.read::<u8>(REG_RW_INTERRUPTSTATUS) & BIT_INTERRUPTMASK_RDC == 0 { 
            /* Busy loop */
        }
        return result;
    }
}

impl NetworkDriver for Ne2kPciDriver {}
