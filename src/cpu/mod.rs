pub mod instructions;
pub mod opcodes;

use crate::bus::Bus;
use std::collections::HashMap;

use bitflags::bitflags;
use instructions::*;

const STACK: u16 = 0x0100;
const STACK_RESET: u8 = 0xFD;

mod interrupt {
    #[derive(PartialEq, Eq)]
    pub enum InterruptType {
        NMI,
        BRK,
    }

    #[derive(PartialEq, Eq)]
    pub(super) struct Interrupt {
        pub(super) itype: InterruptType,
        pub(super) vector_addr: u16,
        pub(super) b_flag_mask: u8,
        pub(super) cpu_cycles: u8,
    }

    pub(super) const NMI: Interrupt = Interrupt {
        itype: InterruptType::NMI,
        vector_addr: 0xFFFA,
        b_flag_mask: 0b00100000,
        cpu_cycles: 2,
    };

    pub(super) const BRK: Interrupt = Interrupt {
        itype: InterruptType::BRK,
        vector_addr: 0xFFFE,
        b_flag_mask: 0b00110000,
        cpu_cycles: 1,
    };
}

bitflags! {
    pub struct StatusFlags: u8 {
        const CARRY     = 0b0000_0001;
        const ZERO      = 0b0000_0010;
        const INTERRUPT = 0b0000_0100;
        const DECIMAL   = 0b0000_1000;
        const BREAK     = 0b0001_0000;
        const UNUSED    = 0b0010_0000;
        const OVERFLOW  = 0b0100_0000;
        const NEGATIVE  = 0b1000_0000;
    }
}

pub struct CPU<'a> {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub sp: u8,
    pub status: StatusFlags,
    pub bus: Bus<'a>,
}

impl<'a> CPU<'a> {
    pub fn new<'b>(bus: Bus<'b>) -> CPU<'b> {
        return CPU {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: STACK_RESET,
            status: StatusFlags::from_bits_truncate(0b100100),
            bus: bus,
        };
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = STACK_RESET;
        self.status = StatusFlags::from_bits_truncate(0b100100);

        self.pc = self.read_reset_vector();
    }

    fn read_reset_vector(&mut self) -> u16 {
        self.bus.read_word(0xFFFC)
    }

    pub fn push(&mut self, value: u8) {
        self.bus.mem_write((STACK as u16) + self.sp as u16, value);
        self.sp = self.sp.wrapping_sub(1);
    }

    pub fn pop(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.bus.mem_read((STACK as u16) + self.sp as u16)
    }

    pub fn push_status(&mut self) {
        let mut status = self.status.bits();
        status |= 0b00110000; // Set the BREAK and unused flags
        self.push(status);
    }

    pub fn push_word(&mut self, value: u16) {
        let hi = (value >> 8) as u8;
        let lo = (value & 0xFF) as u8;
        self.push(hi);
        self.push(lo);
    }

    pub fn fetch_byte(&mut self) -> u8 {
        let byte = self.bus.mem_read(self.pc);
        self.pc += 1;
        byte
    }

    pub fn read_byte(&mut self, addr: u16) -> u8 {
        self.bus.mem_read(addr)
    }

    pub fn fetch_word(&mut self) -> u16 {
        let lo = self.fetch_byte() as u16;
        let hi = self.fetch_byte() as u16;
        (hi << 8) | lo
    }

    pub fn nmi(&mut self) {
        self.push_word(self.pc);
        self.push_status();
        self.status.insert(StatusFlags::INTERRUPT);
        self.pc = self.bus.read_word(0xFFFA);
    }

    fn execute_opcode(&mut self, opcode: u8) {
        match opcode {
            // ADC
            0x69 => self.adc(AddressMode::Immediate),
            0x65 => self.adc(AddressMode::ZeroPage),
            0x75 => self.adc(AddressMode::ZeroPageX),
            0x6D => self.adc(AddressMode::Absolute),
            0x7D => self.adc(AddressMode::AbsoluteX),
            0x79 => self.adc(AddressMode::AbsoluteY),
            0x61 => self.adc(AddressMode::IndirectX),
            0x71 => self.adc(AddressMode::IndirectY),

            // CMP, CPX, CPY
            0xC9 => self.cmp(AddressMode::Immediate),
            0xC5 => self.cmp(AddressMode::ZeroPage),
            0xD5 => self.cmp(AddressMode::ZeroPageX),
            0xCD => self.cmp(AddressMode::Absolute),
            0xDD => self.cmp(AddressMode::AbsoluteX),
            0xD9 => self.cmp(AddressMode::AbsoluteY),
            0xC1 => self.cmp(AddressMode::IndirectX),
            0xD1 => self.cmp(AddressMode::IndirectY),
            0xE0 => self.cpx(AddressMode::Immediate),
            0xE4 => self.cpx(AddressMode::ZeroPage),
            0xEC => self.cpx(AddressMode::Absolute),
            0xC0 => self.cpy(AddressMode::Immediate),
            0xC4 => self.cpy(AddressMode::ZeroPage),
            0xCC => self.cpy(AddressMode::Absolute),

            // AND
            0x29 => self.and(AddressMode::Immediate),
            0x25 => self.and(AddressMode::ZeroPage),
            0x35 => self.and(AddressMode::ZeroPageX),
            0x2D => self.and(AddressMode::Absolute),
            0x3D => self.and(AddressMode::AbsoluteX),
            0x39 => self.and(AddressMode::AbsoluteY),
            0x21 => self.and(AddressMode::IndirectX),
            0x31 => self.and(AddressMode::IndirectY),

            // LDA, LDX, LDY
            0xA9 => self.lda(AddressMode::Immediate),
            0xA5 => self.lda(AddressMode::ZeroPage),
            0xB5 => self.lda(AddressMode::ZeroPageX),
            0xAD => self.lda(AddressMode::Absolute),
            0xBD => self.lda(AddressMode::AbsoluteX),
            0xB9 => self.lda(AddressMode::AbsoluteY),
            0xA1 => self.lda(AddressMode::IndirectX),
            0xB1 => self.lda(AddressMode::IndirectY),
            0xA2 => self.ldx(AddressMode::Immediate),
            0xA6 => self.ldx(AddressMode::ZeroPage),
            0xB6 => self.ldx(AddressMode::ZeroPageY),
            0xAE => self.ldx(AddressMode::Absolute),
            0xBE => self.ldx(AddressMode::AbsoluteY),
            0xA0 => self.ldy(AddressMode::Immediate),
            0xA4 => self.ldy(AddressMode::ZeroPage),
            0xB4 => self.ldy(AddressMode::ZeroPageY),
            0xAC => self.ldy(AddressMode::Absolute),
            0xBC => self.ldy(AddressMode::AbsoluteY),

            // ASL
            0x0A => self.asl_accumulator(),
            0x06 => self.asl(AddressMode::ZeroPage),
            0x16 => self.asl(AddressMode::ZeroPageX),
            0x0E => self.asl(AddressMode::Absolute),
            0x1E => self.asl(AddressMode::AbsoluteX),

            // BCC, BCS, BEQ, BVC, BVS
            0x90 => self.bcc(),
            0xB0 => self.bcs(),
            0xF0 => self.beq(),
            0x30 => self.bmi(),
            0xD0 => self.bne(),
            0x10 => self.bpl(),
            0x50 => self.bvc(),
            0x70 => self.bvs(),

            // BIT
            0x24 => self.bit(AddressMode::ZeroPage),
            0x2C => self.bit(AddressMode::Absolute),

            // DEC, DEX, DEY
            0xC6 => self.dec(AddressMode::ZeroPage),
            0xD6 => self.dec(AddressMode::ZeroPageX),
            0xCE => self.dec(AddressMode::Absolute),
            0xDE => self.dec(AddressMode::AbsoluteX),
            0xCA => self.dex(),
            0x88 => self.dey(),

            // INC
            0xE6 => self.inc(AddressMode::ZeroPage),
            0xF6 => self.inc(AddressMode::ZeroPageX),
            0xEE => self.inc(AddressMode::Absolute),
            0xFE => self.inc(AddressMode::AbsoluteX),
            0xE8 => self.inx(),
            0xC8 => self.iny(),

            // EOR, ORA
            0x49 => self.eor(AddressMode::Immediate),
            0x45 => self.eor(AddressMode::ZeroPage),
            0x55 => self.eor(AddressMode::ZeroPageX),
            0x4D => self.eor(AddressMode::Absolute),
            0x5D => self.eor(AddressMode::AbsoluteX),
            0x59 => self.eor(AddressMode::AbsoluteY),
            0x41 => self.eor(AddressMode::IndirectX),
            0x51 => self.eor(AddressMode::IndirectY),

            0x09 => self.ora(AddressMode::Immediate),
            0x05 => self.ora(AddressMode::ZeroPage),
            0x15 => self.ora(AddressMode::ZeroPageX),
            0x0D => self.ora(AddressMode::Absolute),
            0x1D => self.ora(AddressMode::AbsoluteX),
            0x19 => self.ora(AddressMode::AbsoluteY),
            0x01 => self.ora(AddressMode::IndirectX),
            0x11 => self.ora(AddressMode::IndirectY),

            // LSR
            0x4A => self.lsr_accumulator(),
            0x46 => self.lsr(AddressMode::ZeroPage),
            0x56 => self.lsr(AddressMode::ZeroPageX),
            0x4E => self.lsr(AddressMode::Absolute),
            0x5E => self.lsr(AddressMode::AbsoluteX),

            // JMP
            0x4C => self.jmp(AddressMode::Absolute),
            0x6C => self.jmp(AddressMode::Indirect),
            0x20 => self.jsr(),

            // BRK, CLC, CLC, CLD, CLI, CLV
            0x00 => self.brk(),
            0x18 => self.clc(),
            0xD8 => self.cld(),
            0x58 => self.cli(),
            0xB8 => self.clv(),

            0xEA => self.nop(),

            // PHA, PHP, PLA, PLP
            0x48 => self.pha(),
            0x08 => self.php(),
            0x68 => self.pla(),
            0x28 => self.plp(),

            // ROR, ROL
            0x6A => self.ror_accumulator(),
            0x66 => self.ror(AddressMode::ZeroPage),
            0x76 => self.ror(AddressMode::ZeroPageX),
            0x6E => self.ror(AddressMode::Absolute),
            0x7E => self.ror(AddressMode::AbsoluteX),

            0x2A => self.rol_accumulator(),
            0x26 => self.rol(AddressMode::ZeroPage),
            0x36 => self.rol(AddressMode::ZeroPageX),
            0x2E => self.rol(AddressMode::Absolute),
            0x3E => self.rol(AddressMode::AbsoluteX),

            // RTI
            0x40 => self.rti(),
            0x60 => self.rts(),

            0xE9 => self.sbc(AddressMode::Immediate),
            0xE5 => self.sbc(AddressMode::ZeroPage),
            0xF5 => self.sbc(AddressMode::ZeroPageX),
            0xED => self.sbc(AddressMode::Absolute),
            0xFD => self.sbc(AddressMode::AbsoluteX),
            0xF9 => self.sbc(AddressMode::AbsoluteY),
            0xE1 => self.sbc(AddressMode::IndirectX),
            0xF1 => self.sbc(AddressMode::IndirectY),

            // SEC, SED, SEI
            0x38 => self.sec(),
            0xF8 => self.sed(),
            0x78 => self.sei(),

            // STA
            0x85 => self.sta(AddressMode::ZeroPage),
            0x95 => self.sta(AddressMode::ZeroPageX),
            0x8D => self.sta(AddressMode::Absolute),
            0x9D => self.sta(AddressMode::AbsoluteX),
            0x99 => self.sta(AddressMode::AbsoluteY),
            0x81 => self.sta(AddressMode::IndirectX),
            0x91 => self.sta(AddressMode::IndirectY),

            // STX, STY
            0x86 => self.stx(AddressMode::ZeroPage),
            0x96 => self.stx(AddressMode::ZeroPageY),
            0x8E => self.stx(AddressMode::Absolute),

            0x84 => self.sty(AddressMode::ZeroPage),
            0x94 => self.sty(AddressMode::ZeroPageX),
            0x8C => self.sty(AddressMode::Absolute),

            // TAX, TAY, TSX, TXA, TXS, TYA
            0xAA => self.tax(),
            0xA8 => self.tay(),
            0xBA => self.tsx(),
            0x8A => self.txa(),
            0x9A => self.txs(),
            0x98 => self.tya(),
            _ => self.nop(),
        }
    }

    fn interrupt(&mut self, interrupt: interrupt::Interrupt) {
        self.push_word(self.pc);
        let mut flag = self.status.clone();
        flag.set(StatusFlags::BREAK, interrupt.b_flag_mask & 0b010000 == 1);
        flag.set(StatusFlags::UNUSED, interrupt.b_flag_mask & 0b100000 == 1);

        self.push(flag.bits);
        self.status.insert(StatusFlags::INTERRUPT);

        self.bus.tick(interrupt.cpu_cycles);
        self.pc = self.bus.read_word(interrupt.vector_addr);
    }

    pub fn step(&mut self) {
        let ref opcodes: HashMap<u8, &'static opcodes::OpCode> = *opcodes::OPCODES_MAP;

        let opcode = opcodes
            .get(&self.fetch_byte())
            .expect(&format!("Opcode is not recognized"));

        let pc_state = self.pc;
        println!("{:#02X} {}", self.pc, opcode.mnemonic);
        self.execute_opcode(opcode.code);

        self.bus.tick(opcode.cycles as u8);

        if pc_state == self.pc {
            self.pc += (opcode.len - 1) as u16;
        }
    }

    pub fn run(&mut self) {
        self.run_with_callback(|_| {})
    }

    pub fn run_with_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut CPU),
    {
        let ref opcodes: HashMap<u8, &'static opcodes::OpCode> = *opcodes::OPCODES_MAP;

        loop {
            if let Some(_nmi) = self.bus.ppu.poll_nmi_interrupt() {
                self.interrupt(interrupt::NMI);
            }

            callback(self);

            let opcode = opcodes
                .get(&self.fetch_byte())
                .expect(&format!("Opcode is not recognized"));

            let pc_state = self.pc;
            println!("{:#02X} {}", self.pc, opcode.mnemonic);
            self.execute_opcode(opcode.code);

            self.bus.tick(opcode.cycles as u8);

            if pc_state == self.pc {
                self.pc += (opcode.len - 1) as u16;
            }
        }
    }
}
