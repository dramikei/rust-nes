mod mapper;
mod mapper000;
mod cartridge_header;
mod cartridge_data;
use mapper::Mapper;
use mapper000::Mapper000;
use cartridge_header::CartridgeHeader;
use cartridge_data::CartridgeData;

pub struct Cartridge {
    pub header: CartridgeHeader,
    data: CartridgeData,
    mapper: Box<dyn Mapper>,
}

impl Cartridge {
    pub fn new(data: &[u8]) -> Self {
        let mapper = (data[6] >> 4) | (data[7] & 0xf0);
        let prg_ram_pages = if data[8] == 0 { 1 } else { data[8] as usize };
        
        let header = CartridgeHeader::new(mapper, data[4] as usize, prg_ram_pages, data[5] as usize);
        let cart_data = CartridgeData::new(data[header.prg_rom_range()].to_vec(), vec![0u8; header.prg_ram_bytes()] ,data[header.chr_rom_range()].to_vec(), vec![0u8; header.chr_ram_bytes()]);
        
        //Check for the type of mapper and copy header in the specific mapper's constructor.
        let mapper: Box<dyn Mapper> = match header.mapper_number {
            0 => Box::new(Mapper000::new(header)),
            n => panic!("Mapper {} not implemented", n),
        };
        
        Cartridge {
            header: header,
            data: cart_data,
            mapper: mapper,
        }
    }

    pub fn can_read_addr(&self, addr: u16) -> bool {
        self.mapper.can_read(addr)
    }

    pub fn cpu_read(&self, addr: u16) -> u8 {
        let mapped_addr = self.mapper.read_prg_mapped(addr);
        self.data.prg_rom[mapped_addr as usize]
    }

    pub fn ppu_read(&self, addr: u16) -> u8 {
        self.mapper.read_chr_mapped(addr)
    }
}