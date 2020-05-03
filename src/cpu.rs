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

pub enum Mode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    Implied
}

pub enum Interrupt {
    Reset,
    Irq,
    Nmi,
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

    pub fn clock(&mut self) {
        if self.cycles == 0 {
            let opcode = self.read(self.pc);
            self.set_unused(); //Always true
            self.pc += 1;
            match opcode {
                0x00 => self.brk(),
                0x01 => self.ora(Mode::IndirectX),
                0x05 => self.ora(Mode::ZeroPage),
                0x06 => self.asl(Mode::ZeroPage),
                0x08 => self.php(Mode::Implied),
                0x09 => self.ora(Mode::Immediate),
                0x0a => self.asl_a(),
                0x0d => self.ora(Mode::Absolute),
                0x0e => self.asl(Mode::Absolute),
                0x10 => self.bpl(Mode::Relative),
                0x11 => self.ora(Mode::IndirectY),
                0x15 => self.ora(Mode::ZeroPageX),
                0x16 => self.asl(Mode::ZeroPageX),
                0x18 => self.clc(Mode::Implied),
                0x19 => self.ora(Mode::AbsoluteY),
                0x1d => self.ora(Mode::AbsoluteX),
                0x1e => self.asl(Mode::AbsoluteX),
                0x20 => self.jsr(Mode::Absolute),
                0x21 => self.and(Mode::IndirectX),
                0x24 => self.bit(Mode::ZeroPage),
                0x25 => self.and(Mode::ZeroPage),
                0x26 => self.rol(Mode::ZeroPage),
                0x28 => self.plp(Mode::Implied),
                0x29 => self.and(Mode::Immediate),
                0x2a => self.rol_a(),
                0x2c => self.bit(Mode::Absolute),
                0x2d => self.and(Mode::Absolute),
                0x2e => self.rol(Mode::Absolute),
                0x30 => self.bmi(Mode::Relative),
                0x31 => self.and(Mode::IndirectY),
                0x35 => self.and(Mode::ZeroPageX),
                0x36 => self.rol(Mode::ZeroPageX),
                0x38 => self.sec(Mode::Implied),
                0x39 => self.and(Mode::AbsoluteY),
                0x3d => self.and(Mode::AbsoluteX),
                0x3e => self.rol(Mode::AbsoluteX),
                0x40 => self.rti(Mode::Implied),
                0x41 => self.eor(Mode::IndirectX),
                0x45 => self.eor(Mode::ZeroPage),
                0x46 => self.lsr(Mode::ZeroPage),
                0x48 => self.pha(Mode::Implied),
                0x49 => self.eor(Mode::Immediate),
                0x4a => self.lsr_a(),
                0x4c => self.jmp(Mode::Absolute),
                0x4d => self.eor(Mode::Absolute),
                0x4e => self.lsr(Mode::Absolute),
                0x50 => self.bvc(Mode::Relative),
                0x51 => self.adc(Mode::IndirectX),
                0x55 => self.eor(Mode::ZeroPageX),
                0x56 => self.lsr(Mode::ZeroPageX),
                0x58 => self.cli(Mode::Implied),
                0x59 => self.eor(Mode::AbsoluteY),
                0x5d => self.eor(Mode::AbsoluteX),
                0x5e => self.lsr(Mode::AbsoluteX),
                0x60 => self.rts(Mode::Implied),
                0x61 => self.adc(Mode::IndirectX),
                0x65 => self.adc(Mode::ZeroPage),
                0x66 => self.ror(Mode::ZeroPage),
                0x68 => self.pla(Mode::Implied),
                0x69 => self.adc(Mode::Immediate),
                0x6a => self.ror_a(),
                0x6c => self.jmp(Mode::Indirect),
                0x6d => self.adc(Mode::Absolute),
                0x6e => self.ror(Mode::Absolute),
                

                _ => panic!("Unimplemented OPCODE: {:04x}",opcode)
            }
            
            self.set_unused();
        }
        self.cycles -= 1;
    }

    //Instructions.
    fn ora(&mut self, mode: Mode) {
        let operand: u8 = self.read_operand(&mode);
        let result = self.a | operand;
        self.set_zero(result == 0x00);
        self.set_negative((result & 0x80) > 0);
        self.a = result as u8;
    }

    fn asl(&mut self, mode: Mode) {
        let address = self.operand_address(&mode);
        let operand = self.bus.read(address);
        let result: u16 = (operand as u16) << 1;
        self.set_carry(result > 0xff);
        self.set_zero((result & 0x00ff) == 0x00);
        self.set_negative((result & 0x80) > 0);
        self.write(address, result as u8);
    }

    fn asl_a(&mut self) {
        let operand = self.a;
        let result: u16 = (operand as u16) << 1;
        self.set_carry(result > 0xff);
        self.set_zero((result & 0x00ff) == 0x00);
        self.set_negative((result & 0x80) > 0);
        self.a = result as u8;
    }

    fn php(&mut self, mode: Mode) {
        self.set_break(true);
        self.set_unused();
        self.push_to_stack(self.p);
        self.set_break(false);
    }

    fn bpl(&mut self, mode: Mode) {
        if !self.get_negative() { self.branch() };
    }

    fn clc(&mut self, mode: Mode) {
        self.set_carry(false);
    }

    fn jsr(&mut self, mode: Mode) {
        let x = Mode::Absolute;
        let target_address = self.operand_address(&x);
        let return_address = self.pc - 1;
        self.push_to_stack((return_address >> 8) as u8);
        self.push_to_stack(return_address as u8);
        self.pc = target_address;
    }

    fn and(&mut self, mode: Mode) {
        let operand = self.read_operand(&mode);
        let result = self.a & operand;
        self.set_zero(result == 0x0);
        self.set_negative((result & 0x80) > 0);
        self.a = result;
    }

    fn bit(&mut self, mode: Mode) {
        let operand = self.read_operand(&mode);
        let result = self.a & operand;
        self.set_zero(result == 0);
        self.set_overflow((operand & 0b01000000) != 0);
        self.set_negative((result & 0x80) > 0);
    }

    fn rol(&mut self, mode: Mode) {
        let address = self.operand_address(&mode);
        let operand = self.read(address);
        let carry: u8;
        if self.get_carry() { carry = 1 } else { carry = 0 };
        let result = (operand << 1) | carry;
        self.set_carry(operand & 0b10000000 != 0);
        self.set_zero(result == 0);
        self.set_negative((result & 0x80) > 0);
        self.write(address, result);
    }

    fn rol_a(&mut self) {
        let operand = self.a;
        let carry: u8;
        if self.get_carry() { carry = 1 } else { carry = 0 };
        let result = (operand << 1) | carry;
        self.set_carry(operand & 0b10000000 != 0);
        self.set_zero(result == 0);
        self.set_negative((result & 0x80) > 0);
        self.a = result;
    }

    fn plp(&mut self, mode: Mode) {
        self.p = self.pop_from_stack();
        self.set_unused();
    }

    fn bmi(&mut self, mode: Mode) {
        if self.get_negative() { self.branch() }
    }

    fn sec(&mut self, mode: Mode) {
        self.set_carry(true);
    }

    fn rti(&mut self, mode: Mode) {
        self.p = self.pop_from_stack();
        self.pc = self.pop_from_stack() as u16;
        self.pc |= (self.pop_from_stack() as u16) << 8;
    }

    fn eor(&mut self, mode: Mode) {
        let operand = self.read_operand(&mode);
        let result = self.a ^ operand;
        self.set_zero(result == 0);
        self.set_negative((result & 0x80) > 0);
        self.a = result;
    }

    fn lsr(&mut self, mode: Mode) {
        let address = self.operand_address(&mode);
        let operand = self.read(address);
        let result = operand >> 1;
        self.set_carry(operand & 0b10000000 != 0);
        self.set_zero(result == 0);
        self.set_negative((result & 0x80) > 0);
        self.write(address, result);
    }

    fn lsr_a(&mut self) {
        let operand = self.a;
        let result = operand >> 1;
        self.set_carry(operand & 0b10000000 != 0);
        self.set_zero(result == 0);
        self.set_negative((result & 0x80) > 0);
        self.a = result;
    }

    fn pha(&mut self, mode: Mode) {
        self.push_to_stack(self.a);
    }

    fn jmp(&mut self, mode: Mode) {
        self.pc = self.operand_address(&mode);
    }

    fn bvc(&mut self, mode: Mode) {
        if self.get_overflow() {
            self.branch();
        }
    }

    fn adc(&mut self, mode: Mode) {
        let a = self.a;
        let operand = self.read_operand(&mode);
        let carry: u8;
        if self.get_carry() { carry = 1 } else { carry = 0 };
        let result = a as u16 + operand as u16 + carry as u16;
        self.set_overflow((a as u16 ^ result) & (operand as u16 ^ result) & 0x80 != 0);
        self.set_carry(operand & 0b10000000 != 0);
        self.set_zero(result == 0);
        self.set_negative((result & 0x80) > 0);
        self.a = result as u8;
    }

    fn cli(&mut self, mode: Mode) {
        self.set_interrupt_disable(false);
    }

    fn rts(&mut self, mode: Mode) {
        self.pc = self.pop_from_stack() as u16;
        self.pc |= (self.pop_from_stack() as u16) << 8;
        self.pc +=1;
    }

    fn ror(&mut self, mode: Mode) {
        let address = self.operand_address(&mode);
        let operand = self.read(address);
        let carry: u8;
        if self.get_carry() { carry = 1 } else { carry = 0 };
        let result = (operand >> 1) | (carry << 7);
        self.set_carry(operand & 1 != 0);
        self.set_zero(result == 0);
        self.set_negative((result & 0x80) > 0);
        self.write(address, result);
    }

    fn ror_a(&mut self) {
        let operand = self.a;
        let carry:u8;
        if self.get_carry() { carry = 1 } else { carry = 0 };
        let result = (operand >> 1) | (carry << 7);
        self.set_carry(operand & 1 != 0);
        self.set_zero(result == 0);
        self.set_negative((result & 0x80) > 0);
        self.a = result;
    }

    fn pla(&mut self, mode: Mode) {
        let result = self.pop_from_stack();
        self.set_zero(result == 0);
        self.set_negative((result & 0x80) > 0);
        self.a = result;
    }

    //Helper functions.

    fn branch(&mut self) {
        //TODO: CHECK CYCLES.
        let x = Mode::Immediate;
        let offset = self.read_operand(&x);
        self.pc += offset as u16;
    }

    fn read_operand(&mut self, mode: &Mode) -> u8 {
        let address:u16 = self.operand_address(mode);
        self.read(address)
    }

    fn operand_address(&mut self, mode: &Mode) -> u16 {
        match mode {
            _ => panic!("operand_address called on unimplemented AddressMode!",)
        }
    }

    fn push_to_stack(&mut self, val: u8) {
        self.write(0x100+(self.sp as u16), val);
        self.sp -= 1;
    }

    fn pop_from_stack(&mut self) -> u8 {
        self.sp += 1;
        self.read(0x100+(self.sp as u16))
    }

    //Read are write functions are here to make CPU struct project-independent.
    fn read(&mut self, addr: u16) -> u8 {
        self.bus.read(addr)
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.bus.write(addr, data);
    }

    pub fn interrupt(&mut self, interrupt_type: Interrupt) {
        match interrupt_type {
            Interrupt::Reset => self.reset(),
            Interrupt::Irq => self.irq(),
            Interrupt::Nmi => self.nmi(),
            Interrupt::Break => {
                self.pc -= 1;
                self.brk(); //Calling BREAK instruction. BREAK instruction increments pc by 1 where as interrupt does not.
            }
            _ => panic!("Unimplemented Interrupt called!")
        }
    }

    //private Interrupt functions.
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
        if self.get_interrupt_disable() == false {
            self.push_to_stack((self.pc >> 8) as u8);
            self.push_to_stack(self.pc as u8);

            self.set_break(false);
            self.set_unused();
            self.set_interrupt_disable(true);
            self.push_to_stack(self.p);

            let lo = self.read(0xFFFE);
            let hi = self.read(0xFFFE+1);
            self.pc = ((hi as u16) << 8) | lo as u16;
            self.cycles = 7;
        }
    }

    fn nmi(&mut self) {
        self.push_to_stack((self.pc >> 8) as u8);
        self.push_to_stack(self.pc as u8);

        self.set_break(false);
        self.set_unused();
        self.set_interrupt_disable(true);
        self.push_to_stack(self.p);

        let lo = self.read(0xFFFA);
        let hi = self.read(0xFFFA+1);
        self.pc = ((hi as u16) << 8) | lo as u16;
        self.cycles = 8;
    }

    fn brk(&mut self) {
        self.pc += 1;
        self.set_interrupt_disable(true);
        self.push_to_stack((self.pc >> 8) as u8);
        self.push_to_stack(self.pc as u8);

        self.set_break(true);
        self.push_to_stack(self.p);
        self.set_break(false);
        self.pc = (self.read(0xffff) as u16) << 8 | self.read(0xfffe) as u16;
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

    pub fn set_unused(&mut self) {
        self.p |= 0b00100000;
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
