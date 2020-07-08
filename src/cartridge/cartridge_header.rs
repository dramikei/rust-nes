use std::ops::Range;
const PRG_ROM_PAGE_SIZE: usize = 0x4000; // 16384 * x Bytes
const PRG_RAM_PAGE_SIZE: usize = 0x2000;
const CHR_ROM_PAGE_SIZE: usize = 0x2000; // 8192 * y Bytes
const CHR_RAM_PAGE_SIZE: usize = 0x2000;

pub struct CartridgeHeader {
    pub mapper_number: u8,
    // pub mirroring: Mirroring,
    pub prg_rom_pages: usize,
    pub prg_ram_pages: usize,
    pub chr_rom_pages: usize,
}

impl CartridgeHeader {
    pub fn new(mapper: u8, prg_rom_pages: usize, prg_ram_pages: usize, chr_rom_pages: usize) -> Self {
        CartridgeHeader {
            mapper_number: mapper,
            prg_rom_pages: prg_rom_pages,
            prg_ram_pages: prg_ram_pages,
            chr_rom_pages: chr_rom_pages,
        }
    }

    pub fn prg_rom_range(&self) -> Range<usize> {
        16..16 + self.prg_rom_bytes()
    }

    pub fn chr_rom_range(&self) -> Range<usize> {
        let prg_range = self.prg_rom_range();
        prg_range.end..prg_range.end + self.chr_rom_bytes()
    }

    pub fn prg_rom_bytes(&self) -> usize {
        self.prg_rom_pages * PRG_ROM_PAGE_SIZE
    }

    pub fn prg_ram_bytes(&self) -> usize {
        self.prg_ram_pages * PRG_RAM_PAGE_SIZE
    }

    pub fn chr_rom_bytes(&self) -> usize {
        self.chr_rom_pages * CHR_ROM_PAGE_SIZE
    }

    pub fn chr_ram_bytes(&self) -> usize {
        if self.chr_rom_pages == 0 {
            CHR_RAM_PAGE_SIZE
        } else {
            0
        }
    }
 }