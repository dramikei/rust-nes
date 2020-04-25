use crate::bus::BUS;
//  P -> Flag registor, each bit represents a flag.
//  00000001 -> Carry Flag
//  00000010 -> Zero flag
//  00000100 -> Interrupt Disable
//  00001000 -> Decimal Mode (Doesnt Matter in NES as BCD Instructions are dropped in NES's CPU)
//  00010000 -> Break Command
//  00100000 -> Unused bit
//  01000000 -> Overflow flag
//  10000000 -> Negative flag

enum Mode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    IndirectYForceTick,
    NoMode,
}

pub enum Interrupt {
    Nmi,
    Reset,
    Irq,
    Break,
}

pub struct CPU {
    pub bus: BUS,
    cycles: usize,
    pub a:u8,
    pub x:u8,
    pub y:u8,

    pub p:u8,
    pub sp:u8,
    pub pc:u16,
    
}

//TODO: Check if flags work correctly. Places of problems: Reset() set_*FLAG*()
impl CPU {
    pub fn new(bus: BUS) -> CPU {
        CPU{
            bus,
            cycles:0,
            a:0,
            x:0,
            y:0,

            p:0b00100000,
            sp:0,
            pc:0,
        }
    }

    //Read are write functions are here to make CPU struct project-independent.
    fn read(&mut self, addr: u16) -> u8 {
        self.bus.read(addr)
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.bus.write(addr, data);
    }

    pub fn clock(&mut self) {
        if self.cycles == 0 {
            let opcode = self.read(self.pc);
            self.pc += 1;

        }
    }

    pub fn interrupt(&mut self, interrupt_type: Interrupt) {
        match interrupt_type {
            Interrupt::Reset => self.reset(),
            Interrupt::Irq => self.irq(),
            _ => panic!("Unimplemented Interrupt called!")
        }
    }

    fn reset(&mut self) {
        let lo: u8 = self.read(0xFFFC);
        let hi: u8 = self.read(0xFFFC + 1);
        self.pc = (hi as u16) << 8 | (lo as u16);

        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xFD;
        self.p = 0b00100100;

        //Reset takes time.
        self.cycles = 8;
    }

    fn irq(&mut self) {
        //TODO: implement Push/POP to stack.
        if self.get_interrupt_disable() == false {
            self.write(0x0100 + (self.sp as u16), (self.pc >> 8) as u8);
            self.sp -= 1;
            self.write(0x0100 + (self.sp as u16), self.pc as u8);
            self.sp -= 1;

            self.set_break(false);
            self.set_unused(true);
            self.set_interrupt_disable(true);
            self.write(0x0100 + (self.sp as u16), self.p);
            self.sp -= 1;

            let lo = self.read(0xFFFE);
            let hi = self.read(0xFFFE+1);
            self.pc = ((hi as u16) << 8) | lo as u16;
            self.cycles = 7;
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
    pub fn get_unsed(&mut self) -> bool {
        if (self.p & 0b00100000) == 1 { return true } else { return false };
    }
    pub fn get_overflow(&mut self) -> bool {
        if (self.p & 0b01000000) == 1 { return true } else { return false };
    }
    pub fn get_negative(&mut self) -> bool {
        if (self.p & 0b10000000) == 1 { return true } else { return false };
    }

    //Set Flags
    pub fn set_carry(&mut self, val: bool) {
        if val {
            self.p |= 0b00000001;
        } else {
            self.p &= !0b00000001;
        }
    }

    pub fn set_zero(&mut self, val: bool) {
        if val {
            self.p |= 0b00000010;    
        } else {
            self.p &= !0b00000010;
        }
    }

    pub fn set_interrupt_disable(&mut self, val: bool) {
        if val {
            self.p |= 0b00000100;    
        } else {
            self.p &= !0b00000100;
        }
    }

    pub fn set_break(&mut self, val: bool) {
        if val {
            self.p |= 0b00010000;    
        } else {
            self.p &= !0b00010000;
        }
    }

    pub fn set_unused(&mut self, val: bool) {
        if val {
            self.p |= 0b00100000;    
        } else {
            self.p &= !0b00100000;
        }
    }

    pub fn set_overflow(&mut self, val: bool) {
        if val {
            self.p |= 0b01000000;    
        } else {
            self.p &= !0b01000000;
        }
    }

    pub fn set_negative(&mut self, val: bool) {
        if val {
            self.p |= 0b10000000;
        } else {
            self.p &= !0b10000000;
        }
    }

}
