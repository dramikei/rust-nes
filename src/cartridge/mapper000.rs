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
    fn map_prg_read(&self, address: u16) -> u16 {
	// if PRGROM is 16KB
	//     CPU Address Bus                PRG ROM
	//     0x8000 -> 0xBFFF: Mapped to    0x0000 -> 0x3FFF
	//     0xC000 -> 0xFFFF: Mirror       0x0000 -> 0x3FFF
	// if PRGROM is 32KB
	//     CPU Address Bus                PRG ROM
    //     0x8000 -> 0xFFFF: Mapped to    0x0000 -> 0x7FFF
    // Same with Write
    
        let mapped_addr = address & ( if self.header.prg_rom_pages > 1 { 0x7FFF } else { 0x3FFF });
        mapped_addr
    }

    fn map_prg_write(&mut self, address: u16) -> u16 {
        let mapped_addr = address & ( if self.header.prg_rom_pages > 1 { 0x7FFF } else { 0x3FFF });
        mapped_addr
    }
    fn map_chr_read(&self, address: u16) -> u16 {
        // There is no mapping required for PPU
        // PPU Address Bus                CHR ROM
        // 0x0000 -> 0x1FFF: Mapped to    0x0000 -> 0x1FFF
        // Same with Write
        
        if address > 0x1FFF {panic!("Error: Attempted CHR read beyond 0x1FFF using Mapper000");}
        address
    }
    fn map_chr_write(&mut self, address: u16) -> u16 {
        if address > 0x1fff {
            panic!("Error: Attempted CHR write beyond 0x1FFF using Mapper000");
        } else if self.header.chr_rom_pages != 0 {
            panic!("Error: CHR Page > 0 using Mapper000");
        } else {
            address
        }
    }
    fn irq_flag(&self) -> bool {
        false
    }

    //Can CPU read Address (Unmapped)
    fn can_cpu_read(&self, address: u16) -> bool {
        address >= 0x6000
    }

    //Can PPU read Address (Unmapped)
    fn can_ppu_read(&self, address: u16) -> bool {
        address <= 0x1FFF
    }
}