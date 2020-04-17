const MEM_SIZE: usize = 0x10000;

//  P -> Flag registor, each bit represents a flag.
//  00000001 -> Carry Flag
//  00000010 -> Zero flag
//  00000100 -> Interrupt Disable
//  00001000 -> Decimal Mode (Doesnt Matter in NES as BCD Instructions are dropped in NES's CPU)
//  00010000 -> Break Command
//  00100000 -> Unused bit
//  01000000 -> Overflow flag
//  10000000 -> Negative flag


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

pub struct CPU {
    pub a:u8,
    pub x:u8,
    pub y:u8,

    pub p:u8,
    pub sp:u8,
    pub pc:u16,
    pub memory: [u8;MEM_SIZE]

}

impl CPU {
    pub fn new() -> CPU {
        CPU{
            a:0,
            x:0,
            y:0,

            p:0b00000100,
            sp:0,
            pc:0,
            memory:[0;MEM_SIZE]

        }
    }

    //Get Flags
    pub fn get_carry(&mut self) -> bool {
        if (self.p & 0b00000001) == 1 { return true } else { return false };
    }

    pub fn get_zero(&mut self) -> bool {
        if (self.p & 0b00000010) == 1 { return true } else { return false };
    }
    pub fn get_interrupt_disable(&mut self) -> bool {
        if (self.p & 0b00000100) == 1 { return true } else { return false };
    }
    pub fn get_break(&mut self) -> bool {
        if (self.p & 0b00010000) == 1 { return true } else { return false };
    }
    pub fn get_overflow(&mut self) -> bool {
        if (self.p & 0b01000000) == 1 { return true } else { return false };
    }
    pub fn get_negative(&mut self) -> bool {
        if (self.p & 0b10000000) == 1 { return true } else { return false };
    }

    //Set Flags
    pub fn set_carry(&mut self) {
        self.p |= 0b00000001;
    }

    pub fn set_zero(&mut self) {
        self.p |= 0b00000010;
    }

    pub fn set_interrupt_disable(&mut self) {
        self.p |= 0b00000100;
    }

    pub fn set_break(&mut self) {
        self.p |= 0b00010000;
    }

    pub fn set_overflow(&mut self) {
        self.p |= 0b01000000;
    }

    pub fn set_negative(&mut self) {
        self.p |= 0b10000000;
    }

    //Unset Flags
    pub fn unset_carry(&mut self) {
        self.p &= !0b00000001;
    }

    pub fn unset_zero(&mut self) {
        self.p &= !0b00000010;
    }

    pub fn unset_interrupt_disable(&mut self) {
        self.p &= !0b00000100;
    }

    pub fn unset_break(&mut self) {
        self.p &= !0b00010000;
    }

    pub fn unset_overflow(&mut self) {
        self.p &= !0b01000000;
    }

    pub fn unset_negative(&mut self) {
        self.p &= !0b10000000;
    }

}