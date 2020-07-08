mod mapper;
mod mapper000;
use mapper::Mapper;
use mapper000::Mapper000;

pub struct Cartridge {
    mapper: Box<Mapper>
}

impl Cartridge {
    fn new() -> Self {
        Cartridge {
            mapper: Box::new(Mapper000::new()),
        }
    }
    pub fn can_read_addr(&self, addr: u16) -> bool {
        false
    }

    pub fn cpu_read(&self, addr: u16) -> u8 {
        self.mapper.read_prg_mapped(addr)
    }

    pub fn ppu_read(&self, addr: u16) -> u8 {
        self.mapper.read_chr_mapped(addr)
    }
}