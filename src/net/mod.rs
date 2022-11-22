use core::fmt::Display;

pub mod e1000;
pub mod ne2k_pci;

pub trait NetworkDriver {}

#[derive(Clone, Copy)]
pub struct MacAddress {
    addr: [u8; 6],
}

impl MacAddress {
    pub fn new(addr: [u8; 6]) -> Self {
        Self { addr }
    }
}

impl Display for MacAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for i in 0..5 {
            write!(f, "{:02x}:", self.addr[i])?;
        }
        write!(f, "{:02x}", self.addr[5])
    }
}
