#[path = "cartridge/mod.rs"]
mod cartridge;
use cartridge::Cartridge;
const MEM_SIZE: usize = 2048;
use std::fs;

// Memory
// ========
// 0x100    => Zero Page
// 0x200    => Stack
// 0x800    => RAM
// 0x2000   => Mirrors (0-0x7FF)
// 0x2008   => I/O Registers
// 0x4000   => Mirrors (0x2000-0x2007)
// 0x4020   => I/O Registers
// 0x6000   => Expansion ROM
// 0x8000   => SRAM
// 0xC000   => PRG-ROM (Lower Bank)
// 0x10000  => PRG-ROM (Upper Bank)


pub struct BUS {
    
    pub memory: [u8;MEM_SIZE],
    pub cartridge: Option<Cartridge>,
    pub system_clock_count: usize,
}

impl BUS {
    pub fn new() -> Self {
        BUS {
            memory: [0;MEM_SIZE],
            cartridge: None,
            system_clock_count: 0,
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        if self.cartridge.is_some() && self.cartridge.as_ref().unwrap().can_cpu_read(addr) {
            // Cartridge Address Range
            self.cartridge.as_ref().unwrap().cpu_read(addr)
        } else if addr <= 0x1FFF {
            // System RAM Address Range, mirrored every 2048
            self.memory[(addr & 0x07FF) as usize]
        } else if addr >= 0x2000 && addr <= 0x3FFF {
            // PPU Address range, mirrored every 8
            0 //TODO: Return ppu read data
        } else {
            panic!("Reading at wrong address from bus! {:4x}", addr);
        }
    }


    pub fn write(&mut self, addr: u16, data: u8) {
        if addr <= 0x1FFF {
            // System RAM Address Range, mirrored every 2048
            self.memory[(addr & 0x07FF) as usize] = data;
        } else if addr >= 0x2000 && addr <= 0x3FFF {
            // PPU Address range, mirrored every 8
            //TODO: Write data to ppu
        } else {
            panic!("Reading at wrong address from bus! {:4x}", addr);
        }
    }

    pub fn load_cart(&mut self, name: String) {
        let rom = fs::read(name).expect("Error: Cannot read ROM");
        self.cartridge = Some(Cartridge::new(&rom));
    }
}