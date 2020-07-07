use crate::bus::BUS;
use std::fs::OpenOptions;
use std::io::prelude::*;
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
    Implied,
}
#[allow(dead_code)]
pub enum Interrupt {
    Reset,
    Irq,
    Nmi,
    Break,
}

pub struct CPU {
    pub bus: BUS,
    cycles: u8,
    cycle_count: usize,
    pub a: u8,
    pub x: u8,
    pub y: u8,

    pub p: u8,
    pub sp: u8,
    pub pc: u16,
}

const CYCLES_LIST :[u8; 256] = [
        7, 6, 2, 8, 3, 3, 5, 5, 3, 2, 2, 2, 4, 4, 6, 6, //0x00...0x0F
        2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7, //0x10...0x1F
        6, 6, 2, 8, 3, 3, 5, 5, 4, 2, 2, 2, 4, 4, 6, 6, //0x20...0x2F
        2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7, //0x30...0x3F
        6, 6, 2, 8, 3, 3, 5, 5, 3, 2, 2, 2, 3, 4, 6, 6, //0x40...0x4F
        2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7, //0x50...0x5F
        6, 6, 2, 8, 3, 3, 5, 5, 4, 2, 2, 2, 5, 4, 6, 6, //0x60...0x6F
        2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7, //0x70...0x7F
        2, 6, 2, 6, 3, 3, 3, 3, 2, 2, 2, 2, 4, 4, 4, 4, //0x80...0x8F
        2, 6, 2, 6, 4, 4, 4, 4, 2, 5, 2, 5, 5, 5, 5, 5, //0x90...0x9F
        2, 6, 2, 6, 3, 3, 3, 3, 2, 2, 2, 2, 4, 4, 4, 4, //0xA0...0xAF
        2, 5, 2, 5, 4, 4, 4, 4, 2, 4, 2, 4, 4, 4, 4, 4, //0xB0...0xBF
        2, 6, 2, 8, 3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6, //0xC0...0xCF
        2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7, //0xD0...0xDF
        2, 6, 2, 8, 3, 3, 5, 5, 2, 2, 2, 2, 4, 4, 6, 6, //0xE0...0xEF
        2, 5, 2, 8, 4, 4, 6, 6, 2, 4, 2, 7, 4, 4, 7, 7  //0xF0...0xFF
    ];

    #[allow(dead_code)]
impl CPU {
    pub fn new(bus: BUS) -> Self {
        CPU {
            bus,
            cycles: 0,
            cycle_count: 7,
            a: 0,
            x: 0,
            y: 0,

            p: 0b00100000,
            sp: 0xFD,
            pc: 0,
        }
    }

    pub fn clock(&mut self, debug: bool) {
        if self.cycles == 0 {
            let opcode = self.read(self.pc);
            self.cycles = CYCLES_LIST[opcode as usize];
            let op_1 = self.read(self.pc+1);
            let op_2 = self.read(self.pc+2);

            if debug {log_cpu(self.pc, opcode,op_1,op_2,self.a,self.x,self.y,self.p,self.sp,self.cycle_count)};

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
                0x51 => self.eor(Mode::IndirectY),
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
                0x70 => self.bvs(Mode::Relative),
                0x71 => self.adc(Mode::IndirectY),
                0x75 => self.adc(Mode::ZeroPageX),
                0x76 => self.ror(Mode::ZeroPageX),
                0x78 => self.sei(Mode::Implied),
                0x79 => self.adc(Mode::AbsoluteY),
                0x7d => self.adc(Mode::AbsoluteX),
                0x7e => self.ror(Mode::AbsoluteX),
                0x81 => self.sta(Mode::IndirectX),
                0x84 => self.sty(Mode::ZeroPage),
                0x85 => self.sta(Mode::ZeroPage),
                0x86 => self.stx(Mode::ZeroPage),
                0x88 => self.dey(Mode::Implied),
                0x8a => self.txa(Mode::Implied),
                0x8c => self.sty(Mode::Absolute),
                0x8d => self.sta(Mode::Absolute),
                0x8e => self.stx(Mode::Absolute),
                0x90 => self.bcc(Mode::Relative),
                0x91 => self.sta(Mode::IndirectY),
                0x94 => self.sty(Mode::ZeroPageX),
                0x95 => self.sta(Mode::ZeroPageX),
                0x96 => self.stx(Mode::ZeroPageY),
                0x98 => self.tya(Mode::Implied),
                0x99 => self.sta(Mode::AbsoluteY),
                0x9a => self.txs(Mode::Implied),
                0x9d => self.sta(Mode::AbsoluteX),
                0xa0 => self.ldy(Mode::Immediate),
                0xa1 => self.lda(Mode::IndirectX),
                0xa2 => self.ldx(Mode::Immediate),
                0xa4 => self.ldy(Mode::ZeroPage),
                0xa5 => self.lda(Mode::ZeroPage),
                0xa6 => self.ldx(Mode::ZeroPage),
                0xa8 => self.tay(Mode::Implied),
                0xa9 => self.lda(Mode::Immediate),
                0xaa => self.tax(Mode::Implied),
                0xac => self.ldy(Mode::Absolute),
                0xad => self.lda(Mode::Absolute),
                0xae => self.ldx(Mode::Absolute),
                0xb0 => self.bcs(Mode::Relative),
                0xb1 => self.lda(Mode::IndirectY),
                0xb4 => self.ldy(Mode::ZeroPageX),
                0xb5 => self.lda(Mode::ZeroPageX),
                0xb6 => self.ldx(Mode::ZeroPageY),
                0xb8 => self.clv(Mode::Implied),
                0xb9 => self.lda(Mode::AbsoluteY),
                0xba => self.tsx(Mode::Implied),
                0xbc => self.ldy(Mode::AbsoluteX),
                0xbd => self.lda(Mode::AbsoluteX),
                0xbe => self.ldx(Mode::AbsoluteY),
                0xc0 => self.cpy(Mode::Immediate),
                0xc1 => self.cmp(Mode::IndirectX),
                0xc4 => self.cpy(Mode::ZeroPage),
                0xc5 => self.cmp(Mode::ZeroPage),
                0xc6 => self.dec(Mode::ZeroPage),
                0xc8 => self.iny(Mode::Implied),
                0xc9 => self.cmp(Mode::Immediate),
                0xca => self.dex(Mode::Implied),
                0xcc => self.cpy(Mode::Absolute),
                0xcd => self.cmp(Mode::Absolute),
                0xce => self.dec(Mode::Absolute),
                0xd0 => self.bne(Mode::Relative),
                0xd1 => self.cmp(Mode::IndirectY),
                0xd5 => self.cmp(Mode::ZeroPageX),
                0xd6 => self.dec(Mode::ZeroPageX),
                0xd8 => self.cld(Mode::Implied),
                0xd9 => self.cmp(Mode::AbsoluteY),
                0xdd => self.cmp(Mode::AbsoluteX),
                0xde => self.dec(Mode::AbsoluteX),
                0xe0 => self.cpx(Mode::Immediate),
                0xe1 => self.sbc(Mode::IndirectX),
                0xe4 => self.cpx(Mode::ZeroPage),
                0xe5 => self.sbc(Mode::ZeroPage),
                0xe6 => self.inc(Mode::ZeroPage),
                0xe8 => self.inx(Mode::Implied),
                0xe9 => self.sbc(Mode::Immediate),
                0xea => self.nop(Mode::Implied),
                0xec => self.cpx(Mode::Absolute),
                0xed => self.sbc(Mode::Absolute),
                0xee => self.inc(Mode::Absolute),
                0xf0 => self.beq(Mode::Relative),
                0xf1 => self.sbc(Mode::IndirectY),
                0xf5 => self.sbc(Mode::ZeroPageX),
                0xf6 => self.inc(Mode::ZeroPageX),
                0xf8 => self.sed(Mode::Implied),
                0xf9 => self.sbc(Mode::AbsoluteY),
                0xfd => self.sbc(Mode::AbsoluteX),
                0xfe => self.inc(Mode::AbsoluteX),

                //////// Undocumented Instructions ////////

                0x0C => self.nop_read(Mode::Absolute),
                0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => self.nop_read(Mode::AbsoluteX),
                0x04 | 0x44 | 0x64 => self.nop_read(Mode::ZeroPage),
                0x14 | 0x34 | 0x54 | 0x74 | 0xd4 | 0xf4 => self.nop_read(Mode::ZeroPageX),
                0x80 | 0x82 | 0x89 | 0xc2 | 0xe2 => self.nop_read(Mode::Immediate),

                0xA7 => self.lax(Mode::ZeroPage),
                0xB7 => self.lax(Mode::ZeroPageY),
                0xA3 => self.lax(Mode::IndirectX),
                0xB3 => self.lax(Mode::IndirectY),
                0xAF => self.lax(Mode::Absolute),
                0xBF => self.lax(Mode::AbsoluteY),

                0x87 => self.sax(Mode::ZeroPage),
                0x97 => self.sax(Mode::ZeroPageY),
                0x83 => self.sax(Mode::IndirectX),
                0x8F => self.sax(Mode::Absolute),

                0xEB => self.sbc(Mode::Immediate),

                0xC7 => self.dcp(Mode::ZeroPage),
                0xD7 => self.dcp(Mode::ZeroPageX),
                0xC3 => self.dcp(Mode::IndirectX),
                0xD3 => self.dcp(Mode::IndirectY),
                0xCF => self.dcp(Mode::Absolute),
                0xDF => self.dcp(Mode::AbsoluteX),
                0xDB => self.dcp(Mode::AbsoluteY),

                0xE7 => self.isc(Mode::ZeroPage),
                0xF7 => self.isc(Mode::ZeroPageX),
                0xE3 => self.isc(Mode::IndirectX),
                0xF3 => self.isc(Mode::IndirectY),
                0xEF => self.isc(Mode::Absolute),
                0xFF => self.isc(Mode::AbsoluteX),
                0xFB => self.isc(Mode::AbsoluteY),

                0x07 => self.slo(Mode::ZeroPage),
                0x17 => self.slo(Mode::ZeroPageX),
                0x03 => self.slo(Mode::IndirectX),
                0x13 => self.slo(Mode::IndirectY),
                0x0F => self.slo(Mode::Absolute),
                0x1F => self.slo(Mode::AbsoluteX),
                0x1B => self.slo(Mode::AbsoluteY),

                0x27 => self.rla(Mode::ZeroPage),
                0x37 => self.rla(Mode::ZeroPageX),
                0x23 => self.rla(Mode::IndirectX),
                0x33 => self.rla(Mode::IndirectY),
                0x2F => self.rla(Mode::Absolute),
                0x3F => self.rla(Mode::AbsoluteX),
                0x3B => self.rla(Mode::AbsoluteY),

                0x47 => self.sre(Mode::ZeroPage),
                0x57 => self.sre(Mode::ZeroPageX),
                0x43 => self.sre(Mode::IndirectX),
                0x53 => self.sre(Mode::IndirectY),
                0x4F => self.sre(Mode::Absolute),
                0x5F => self.sre(Mode::AbsoluteX),
                0x5B => self.sre(Mode::AbsoluteY),

                0x67 => self.rra(Mode::ZeroPage),
                0x77 => self.rra(Mode::ZeroPageX),
                0x63 => self.rra(Mode::IndirectX),
                0x73 => self.rra(Mode::IndirectY),
                0x6F => self.rra(Mode::Absolute),
                0x7F => self.rra(Mode::AbsoluteX),
                0x7B => self.rra(Mode::AbsoluteY),
                _ => self.nop(Mode::Implied),
            }
            self.set_unused();
        }
        self.cycle_count += 1;
        self.cycles -= 1;
    }

    //Instructions.
    fn ora(&mut self, mode: Mode) {
        let operand: u8 = self.read_operand(&mode, true);
        let result = self.a | operand;
        self.set_zero(result == 0x00);
        self.set_negative((result & 0x80) > 0);
        self.a = result as u8;
    }

    fn asl(&mut self, mode: Mode) {
        self.asl_ret(&mode);
    }

    fn asl_a(&mut self) {
        let operand = self.a;
        let result: u16 = (operand as u16) << 1;
        self.set_carry(operand & 0b10000000 != 0);
        self.set_zero((result & 0x00ff) == 0x00);
        self.set_negative((result & 0x80) > 0);
        self.a = result as u8;
    }

    fn asl_ret(&mut self, mode: &Mode) -> u8 {
        let address = self.operand_address(mode, false);
        let operand = self.read(address);
        let result: u16 = (operand as u16) << 1;
        self.set_carry(operand & 0b10000000 != 0);
        self.set_zero((result & 0x00ff) == 0x00);
        self.set_negative((result & 0x80) > 0);
        self.write(address, result as u8);
        result as u8
    }

    fn php(&mut self, _ : Mode) {
        self.set_break(true);
        self.set_unused();
        self.push_to_stack(self.p);
        self.set_break(false);
    }

    fn bpl(&mut self, _ : Mode) {
        let condition = !self.get_negative();
        self.branch(condition);
    }

    fn clc(&mut self, _ : Mode) {
        self.set_carry(false);
    }

    fn jsr(&mut self, _ : Mode) {
        let x = Mode::Absolute;
        let target_address = self.operand_address(&x, false);
        let return_address = self.pc - 1;
        self.push_to_stack((return_address >> 8) as u8);
        self.push_to_stack(return_address as u8);
        self.pc = target_address;
    }

    fn and(&mut self, mode: Mode) {
        let operand = self.read_operand(&mode, true);
        let result = self.a & operand;
        self.set_zero(result == 0x0);
        self.set_negative((result & 0x80) > 0);
        self.a = result;
    }

    fn bit(&mut self, mode: Mode) {
        let operand = self.read_operand(&mode, false);
        let result = self.a & operand;
        self.set_zero((result as u8) == 0);
        self.set_overflow(operand & 0b01000000 != 0);
        self.set_negative(operand & 0b10000000 != 0);
    }

    fn rol(&mut self, mode: Mode) {
        self.rol_ret(&mode);
    }

    fn rol_a(&mut self) {
        let operand = self.a;
        let carry: u8;
        if self.get_carry() { carry = 1 } else { carry = 0 };
        let result = (operand << 1) | carry;
        self.set_carry(operand & 0b10000000 != 0);
        self.set_zero((result as u8) == 0);
        self.set_negative((result & 0x80) > 0);
        self.a = result;
    }

    fn rol_ret(&mut self, mode: &Mode) -> u8 {
        let address = self.operand_address(mode, false);
        let operand = self.read(address);
        let carry: u8;
        if self.get_carry() { carry = 1 } else { carry = 0 };
        let result = (operand << 1) | carry;
        self.set_carry(operand & 0b10000000 != 0);
        self.set_zero((result as u8) == 0);
        self.set_negative((result & 0x80) > 0);
        self.write(address, result);
        result
    }

    fn plp(&mut self, _ : Mode) {
        self.p = self.pop_from_stack();
        self.set_unused();
    }

    fn bmi(&mut self, _ : Mode) {
        let condition = self.get_negative();
        self.branch(condition);
    }

    fn sec(&mut self, _ : Mode) {
        self.set_carry(true);
    }

    fn rti(&mut self, _ : Mode) {
        self.p = self.pop_from_stack();
        self.pc = self.pop_from_stack() as u16;
        self.pc |= (self.pop_from_stack() as u16) << 8;
    }

    fn eor(&mut self, mode: Mode) {
        let operand = self.read_operand(&mode, true);
        let result = self.a ^ operand;
        self.set_zero((result as u8) == 0);
        self.set_negative((result & 0b10000000) != 0);
        self.a = result;
    }

    fn lsr(&mut self, mode: Mode) {
        self.lsr_ret(&mode);
    }

    fn lsr_a(&mut self) {
        let operand = self.a;
        let result = operand >> 1;
        self.set_carry(operand & 1 != 0);
        self.set_zero((result as u8) == 0);
        self.set_negative((result & 0x80) > 0);
        self.a = result;
    }

    fn lsr_ret(&mut self, mode: &Mode) -> u8 {
        let address = self.operand_address(mode, false);
        let operand = self.read(address);
        let result = operand >> 1;
        self.set_carry(operand & 1 != 0);
        self.set_zero((result as u8) == 0);
        self.set_negative((result & 0x80) > 0);
        self.write(address, result);
        result
    }

    fn pha(&mut self, _ : Mode) {
        self.push_to_stack(self.a);
    }

    fn jmp(&mut self, mode: Mode) {
        self.pc = self.operand_address(&mode, false);
        // self.cycles = 7;
    }

    fn bvc(&mut self, _ : Mode) {
        let condition = !self.get_overflow();
        self.branch(condition);
    }

    fn adc(&mut self, mode: Mode) {
        let a = self.a;
        let operand = self.read_operand(&mode, true);
        let carry: u8;
        if self.get_carry() { carry = 1 } else { carry = 0 };
        let result = a as u16 + operand as u16 + carry as u16;
        self.set_overflow((a as u16 ^ result) & (operand as u16 ^ result) & 0x80 != 0);
        self.set_carry(result>0xff);
        self.set_zero((result as u8) == 0);
        self.set_negative((result & 0x80) > 0);
        self.a = result as u8;
    }

    fn cli(&mut self, _ : Mode) {
        self.set_interrupt_disable(false);
    }

    fn rts(&mut self, _ : Mode) {
        self.pc = self.pop_from_stack() as u16;
        self.pc |= (self.pop_from_stack() as u16) << 8;
        self.pc += 1;
    }

    fn ror(&mut self, mode: Mode) {
        self.ror_ret(&mode);
    }

    fn ror_a(&mut self) {
        let operand = self.a;
        let carry: u8;
        if self.get_carry() { carry = 1 } else { carry = 0 };
        let result = (operand >> 1) | (carry << 7);
        self.set_carry(operand & 1 != 0);
        self.set_zero((result as u8) == 0);
        self.set_negative((result & 0x80) > 0);
        self.a = result;
    }

    fn ror_ret(&mut self, mode: &Mode) -> u8 {
        let address = self.operand_address(mode, false);
        let operand = self.read(address);
        let carry: u8;
        if self.get_carry() { carry = 1 } else { carry = 0 };
        let result = (operand >> 1) | (carry << 7);
        self.set_carry(operand & 1 != 0);
        self.set_zero((result as u8) == 0);
        self.set_negative((result & 0x80) > 0);
        self.write(address, result);
        result
    }

    fn pla(&mut self, _ : Mode) {
        let result = self.pop_from_stack();
        self.set_zero((result as u8) == 0);
        self.set_negative((result & 0x80) > 0);
        self.a = result;
    }

    fn bvs(&mut self, _ : Mode) {
        let condition = self.get_overflow();
        self.branch(condition);
    }

    fn sei(&mut self, _ : Mode) {
        self.set_interrupt_disable(true);
    }

    fn sty(&mut self, mode: Mode) {
        let address = self.operand_address(&mode, false);
        let value = self.y;
        self.write(address, value);
    }

    fn sta(&mut self, mode: Mode) {
        let address = self.operand_address(&mode, false);
        let value = self.a;
        self.write(address, value);
    }

    fn stx(&mut self, mode: Mode) {
        let address = self.operand_address(&mode, false);
        let value = self.x;
        self.write(address, value);
    }

    fn dey(&mut self, _ : Mode) {
        let result = self.y.wrapping_sub(1);
        self.set_zero((result as u8) == 0);
        self.set_negative((result & 0x80) > 0);
        self.y = result;
    }

    fn txa(&mut self, _ : Mode) {
        let result = self.x;
        self.set_zero((result as u8) == 0);
        self.set_negative((result & 0x80) > 0);
        self.a = result;
    }

    fn bcc(&mut self, _ : Mode) {
        let condition = !self.get_carry();
        self.branch(condition);
    }

    fn tya(&mut self, _ : Mode) {
        let result = self.y;
        self.set_zero((result as u8) == 0);
        self.set_negative((result & 0x80) > 0);
        self.a = result;
    }

    fn txs(&mut self, _ : Mode) {
        let result = self.x;
        self.sp = result;
    }

    fn lax(&mut self, mode: Mode) {
        self.lda(mode);
        self.x = self.a;
    }

    fn ldy(&mut self, mode: Mode) {
        let operand = self.read_operand(&mode, true);
        self.set_zero(operand == 0);
        self.set_negative((operand & 0x80) > 0);
        self.y = operand;
    }

    fn lda(&mut self, mode: Mode) {
        let operand = self.read_operand(&mode, true);
        self.set_zero(operand == 0);
        self.set_negative((operand & 0x80) > 0);
        self.a = operand;
    }

    fn ldx(&mut self, mode: Mode) {
        let operand = self.read_operand(&mode, true);
        self.set_zero(operand == 0);
        self.set_negative((operand & 0x80) > 0);
        self.x = operand;
    }

    fn tay(&mut self, _ : Mode) {
        let result = self.a;
        self.set_zero((result as u8) == 0);
        self.set_negative((result & 0x80) > 0);
        self.y = result;
    }

    fn tax(&mut self, _ : Mode) {
        let result = self.a;
        self.set_zero((result as u8) == 0);
        self.set_negative((result & 0x80) > 0);
        self.x = result;
    }

    fn bcs(&mut self, _ : Mode) {
        let condition = self.get_carry();
        self.branch(condition);
    }

    fn clv(&mut self, _ : Mode) {
        self.set_overflow(false);
    }

    fn tsx(&mut self, _ : Mode) {
        self.x = self.sp;
        self.set_zero(self.x == 0);
        self.set_negative((self.x & 0x80) > 0);
    }

    fn cpy(&mut self, mode: Mode) {
        let operand = self.read_operand(&mode, false);
        let y = self.y;
        self.set_zero(y.wrapping_sub(operand) == 0);
        self.set_negative((y.wrapping_sub(operand) & 0x80) > 0);
        self.set_carry(y >= operand);
    }

    fn cmp(&mut self, mode: Mode) {
        let operand = self.read_operand(&mode, true);
        let a = self.a;
        self.set_zero(a.wrapping_sub(operand) == 0);
        self.set_negative((a.wrapping_sub(operand) & 0x80) > 0);
        self.set_carry(a >= operand);
    }

    fn dec(&mut self, mode: Mode) {
        let address = self.operand_address(&mode, false);
        let operand = self.read(address);
        let result = operand.wrapping_sub(1);
        self.set_zero((result as u8) == 0);
        self.set_negative((result & 0x80) > 0);
        self.write(address, result);
    }

    fn iny(&mut self, _ : Mode) {
        let result = self.y.wrapping_add(1);
        self.set_zero((result as u8) == 0);
        self.set_negative((result & 0x80) > 0);
        self.y = result;
    }

    fn dex(&mut self, _ : Mode) {
        let result = self.x.wrapping_sub(1);
        self.set_zero((result as u8) == 0);
        self.set_negative((result & 0x80) > 0);
        self.x = result;
    }

    fn bne(&mut self, _ : Mode) {
        let condition = !self.get_zero();
        self.branch(condition);
    }
    fn cld(&mut self, _ : Mode) {
        self.set_decimal(false);
    }

    fn cpx(&mut self, mode: Mode) {
        let operand = self.read_operand(&mode, false);
        let x = self.x;
        self.set_zero(x.wrapping_sub(operand) == 0);
        self.set_negative((x.wrapping_sub(operand) & 0x80) > 0);
        self.set_carry(x >= operand);
    }

    fn sbc(&mut self, mode: Mode) {
        let a = self.a;
        let operand = !self.read_operand(&mode, true);
        let carry: u8;
        if self.get_carry() { carry = 1 } else { carry = 0 };
        let result = a as u16 + operand as u16 + carry as u16;
        self.set_overflow((a as u16 ^ result) & (operand as u16 ^ result) & 0x80 != 0);
        self.set_carry(result>0xff);
        self.set_zero((result as u8) == 0);
        self.set_negative((result & 0x80) > 0);
        self.a = result as u8;
    }

    fn inc(&mut self, mode: Mode) {
        let address = self.operand_address(&mode, false);
        let operand = self.read(address);
        let result = operand.wrapping_add(1);
        self.set_zero((result as u8) == 0);
        self.set_negative((result & 0x80) > 0);
        self.write(address, result);
    }

    fn inx(&mut self, _ : Mode) {
        let result = self.x.wrapping_add(1);
        self.set_zero((result as u8) == 0);
        self.set_negative((result & 0x80) > 0);
        self.x = result;
    }

    fn nop(&mut self, _ : Mode) {}

    fn nop_read(&mut self, mode: Mode) {
        self.read_operand(&mode, true);
    }

    fn beq(&mut self, _ : Mode) {
        let condition = self.get_zero();
        self.branch(condition);
    }

    fn sed(&mut self, _ : Mode) {
        self.set_decimal(true);
    }

    fn sax(&mut self, mode: Mode) {
        let address = self.operand_address(&mode, false);
        let result = self.a & self.x;
        self.write(address, result);
    }

    fn dcp(&mut self, mode: Mode) {
        let address = self.operand_address(&mode, false);
        let operand = self.read(address);
        let result = operand.wrapping_sub(1);
        self.write(address, result);
        let a = self.a;
        self.set_zero((a.wrapping_sub(result) as u8) == 0);
        self.set_negative((a.wrapping_sub(result) & 0b10000000) != 0);
        self.set_carry(a >= result);
    }

    fn isc(&mut self, mode: Mode) {
        //Incrementing
        let address = self.operand_address(&mode, false);
        let operand = self.read(address);
        let result = operand.wrapping_add(1);
        self.write(address, result);
        
        //reasigning operand
        let operand = !result;

        let a = self.a;
        let result = a as u16 + operand as u16 + self.get_carry() as u16;
        self.set_carry(result > 0xFF);
        self.set_overflow((a ^ (result as u8)) & (operand ^ (result as u8)) & 0x80 != 0);
        self.set_zero(result == 0);
        self.set_negative((result & 0b10000000) != 0);
        self.a = result as u8;
    }

    fn slo(&mut self, mode: Mode) {
        let result = self.a | self.asl_ret(&mode);
        self.set_zero((result & 0x00ff) == 0x00);
        self.set_negative((result & 0x80) > 0);
        self.a = result;
    }

    fn rla(&mut self, mode: Mode) {
        let result = self.a & self.rol_ret(&mode);
        self.set_zero((result as u8) == 0);
        self.set_negative((result & 0x80) > 0);
        self.a = result;
    }
    
    fn sre(&mut self, mode: Mode) {
        let result = self.a ^ self.lsr_ret(&mode);
        self.set_zero((result as u8) == 0);
        self.set_negative((result & 0x80) > 0);
        self.a = result;
    }

    fn rra(&mut self, mode: Mode) {
        let a = self.a;
        let operand = self.ror_ret(&mode);
        let result = a as u16 + operand as u16 + self.get_carry() as u16;
        self.set_carry(result > 0xFF);
        self.set_overflow((a ^ (result as u8)) & (operand ^ (result as u8)) & 0x80 != 0);
        self.set_zero((result as u8) == 0);
        self.set_negative((result & 0x80) > 0);
        self.a = result as u8;
    }

    //Helper functions.

    pub fn complete(&mut self) -> bool {
        self.cycles == 0
    }

    fn branch(&mut self, condition: bool) {
        let x = Mode::Immediate;
        let offset = self.read_operand(&x, true) as i8 as u16;
        if condition {
            self.cycles += 1;
            let absolute_addr = self.pc.wrapping_add(offset as u16);
            if (absolute_addr & 0xFF00) != (self.pc & 0xFF00) { self.cycles += 1};
            self.pc = absolute_addr
        }
    }

    fn read_operand(&mut self, mode: &Mode, needs_additional_cycle: bool) -> u8 {
        let address: u16 = self.operand_address(mode, needs_additional_cycle);
        self.read(address)
    }

    fn operand_address(&mut self, mode: &Mode, needs_additional_cycle: bool) -> u16 {
        match mode {
            Mode::Immediate => {
                let original_pc = self.pc;
                self.increment_pc();
                original_pc
            }
            Mode::ZeroPage => self.next_byte() as u16,
            Mode::ZeroPageX => {
                low_byte(offset(self.next_byte(), self.x))
            }
            Mode::ZeroPageY => {
                low_byte(offset(self.next_byte(), self.y))
            }
            Mode::Absolute => self.next_word(),
            Mode::AbsoluteX => {
                let base = self.next_word();
                let res = offset(base, self.x);
                if ((base & 0xFF00) != (res & 0xFF00)) && needs_additional_cycle {
                    self.cycles += 1
                }
                res
            }
            Mode::AbsoluteY => {
                let base = self.next_word();
                let res = offset(base, self.y);
                if ((base & 0xFF00) != (res & 0xFF00)) && needs_additional_cycle {
                    self.cycles += 1
                }
                res
            }
            Mode::Indirect => {
                let i = self.next_word();
                let x = self.read(i);
                let y = self.read(high_byte(i) | low_byte(i + 1));
                return ((y as u16) << 8) | (x as u16);
            }
            Mode::IndirectX => {
                let i = offset(self.next_byte(), self.x);
                let x = self.read(low_byte(i));
                let y = self.read(low_byte(i + 1));
                return ((y as u16) << 8) | (x as u16);
            }
            Mode::IndirectY => {
                let i = self.next_byte();
                let x = self.read(i as u16);
                let y = self.read(low_byte(i.wrapping_add(1)));
                let base = ((y as u16) << 8) | (x as u16);
                let res = offset(base, self.y);
                if ((base & 0xFF00) != (res & 0xFF00)) && needs_additional_cycle {
                    self.cycles += 1
                }
                res
            }
            _ => panic!("Error: Unknown mode to read from memory"),
        }
    }

    fn increment_pc(&mut self) {
        self.pc = self.pc.wrapping_add(1);
    }

    fn next_byte(&mut self) -> u8 {
        let original_pc = self.pc;
        self.increment_pc();
        self.read(original_pc)
    }

    fn next_word(&mut self) -> u16 {
        let original_pc: u16 = self.pc;
        self.increment_pc();
        self.increment_pc();
        let x = self.read(original_pc);
        let y = self.read(original_pc+1);
        return ((y as u16) << 8) | (x as u16);
    }

    fn push_to_stack(&mut self, val: u8) {
        self.write(0x100 + (self.sp as u16), val);
        self.sp -= 1;
    }

    fn pop_from_stack(&mut self) -> u8 {
        self.sp += 1;
        self.read(0x100 + (self.sp as u16))
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
            let hi = self.read(0xFFFE + 1);
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
        let hi = self.read(0xFFFA + 1);
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
        if (self.p & 0b00000001) == 1 {
            return true;
        } else {
            return false;
        };
    }

    pub fn get_zero(&mut self) -> bool {
        if (self.p & 0b00000010) == 0b00000010 {
            return true;
        } else {
            return false;
        };
    }
    pub fn get_interrupt_disable(&mut self) -> bool {
        if (self.p & 0b00000100) == 0b00000100 {
            return true;
        } else {
            return false;
        };
    }
    pub fn get_decimal(&mut self) -> bool {
        if (self.p & 0b00001000) == 0b00001000 {
            return true;
        } else {
            return false;
        };
    }
    pub fn get_break(&mut self) -> bool {
        if (self.p & 0b00010000) == 0b00010000 {
            return true;
        } else {
            return false;
        };
    }
    pub fn get_unsed(&mut self) -> bool {
        if (self.p & 0b00100000) == 0b00100000 {
            return true;
        } else {
            return false;
        };
    }
    pub fn get_overflow(&mut self) -> bool {
        if (self.p & 0b01000000) == 0b01000000 {
            return true;
        } else {
            return false;
        };
    }
    pub fn get_negative(&mut self) -> bool {
        if (self.p & 0b10000000) == 0b10000000 {
            return true;
        } else {
            return false;
        };
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

    pub fn set_decimal(&mut self, val: bool) {
        if val {
            self.p |= 0b00001000;
        } else {
            self.p &= !0b00001000;
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

fn log_cpu(pc: u16, op: u8, op1: u8, op2: u8, a: u8, x: u8, y: u8, p: u8, sp: u8, tot_cyc: usize) {
    //TODO: Improve logging. 
    let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(true)
                    .open("log.txt")
                    .unwrap();

                if let Err(e) = writeln!(file, "{:04X}    {:02X} {:02X} {:02X}         A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} PPU:{}, CYC:{}",pc, op, op1, op2, a, x, y, p, sp, 0, tot_cyc) {
                    panic!("Couldn't write to file: {}", e);
                }
                //Example: C000  4C F5 C5  JMP $C5F5                       A:00 X:00 Y:00 P:24 SP:FD PPU:  0, 21 CYC:7
                println!("{:04x}    {:02x} {:02x} {:02x}         A:{:02x} X:{:02x} Y:{:02x} P:{:02x} SP:{:02x} PPU:{}, CYC:{}",pc, op, op1, op2, a, x, y, p, sp, 0, tot_cyc);
}

fn high_byte(value: u16) -> u16 {
    value & 0xFF00
}

pub fn low_byte<T: Into<u16>>(value: T) -> u16 {
    value.into() & 0xFF
}

fn offset<T: Into<u16>>(base: T, offset: u8) -> u16 {
    base.into().wrapping_add(offset as u16) as u16
}
