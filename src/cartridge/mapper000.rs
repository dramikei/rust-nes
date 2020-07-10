use super::Mapper;
use super::CartridgeHeader;

pub struct Mapper000 {
    pub header: CartridgeHeader
}
impl Mapper000 {
    pub fn new(header: CartridgeHeader) -> Self {
       Mapper000 {
        header: header
       }
    }
}

impl Mapper for Mapper000 {
    fn read_prg_mapped(&self, address: u16) -> u16 {
        let mapped_addr = address & ( if self.header.prg_rom_pages > 1 { 0x7FFF } else { 0x3FFF} );
        mapped_addr
    }

    fn write_prg_mapped(&mut self, address: u16, value: u8) {

    }
    fn read_chr_mapped(&self, address: u16) -> u8 {
        0
    }
    fn write_chr_mapped(&mut self, address: u16, value: u8) {

    }
    fn irq_flag(&self) -> bool {
        false
    }
    fn can_read(&self, address: u16) -> bool {
        address >= 0x6000
    }
}