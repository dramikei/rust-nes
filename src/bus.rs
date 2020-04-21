
const MEM_SIZE: usize = 0x10000;

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
    
    //temp ram
    pub memory: [u8;MEM_SIZE]
}

impl BUS {
    pub fn new() -> BUS {
        BUS {
            memory: [0;MEM_SIZE]
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        //TODO: Change limits when adding new components ot the BUS.
        if addr >= 0x0000 && addr <= 0xffff {
            self.memory[addr as usize]
        } else {
            panic!("Reading at wrong address from bus! {:4x}", addr);
        }
    }


    pub fn write(&mut self, addr: u16, data: u8) {
        //TODO: Change limits when adding new components ot the BUS.
        if addr >= 0x0000 && addr <= 0xffff {
            self.memory[addr as usize] = data;
        }
    }
}