use crate::bus::MemoryBus;
use crate::instructions::AddressMode;
use crate::nes_file::Mirroring;
use bitflags::bitflags;

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

pub struct CPU {
    pub cycles: u16,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub sp: u8,
    pub status: StatusFlags,
    pub memory_bus: MemoryBus,
}

impl CPU {
    pub fn new(prg_rom: Vec<u8>, chr_rom: Vec<u8>, mirroring: Mirroring) -> CPU {
        let mut cpu = CPU {
            cycles: 0,
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0xFD,
            status: StatusFlags::from_bits_truncate(0x34),
            memory_bus: MemoryBus::new(prg_rom, chr_rom, mirroring),
        };

        cpu.pc = cpu.read_reset_vector();
        cpu
    }

    fn read_reset_vector(&mut self) -> u16 {
        let lo = self.memory_bus.cpu_read(0xFFFC) as u16;
        let hi = self.memory_bus.cpu_read(0xFFFD) as u16;
        (hi << 8) | lo
    }

    pub fn push(&mut self, value: u8) {
        self.memory_bus.cpu_write(0x0100 + self.sp as u16, value);
        self.sp = self.sp.wrapping_sub(1);
    }

    pub fn pop(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.memory_bus.cpu_read(0x0100 + self.sp as u16)
    }

    pub fn push_status(&mut self) {
        let mut status = self.status.bits();
        status |= 0b00110000; // Set the BREAK and unused flags
        self.push(status);
    }

    pub fn push_word(&mut self, value: u16) {
        self.push((value >> 8) as u8);
        self.push(value as u8);
    }

    pub fn fetch_byte(&mut self) -> u8 {
        let byte = self.memory_bus.cpu_read(self.pc);
        self.pc += 1;
        byte
    }

    pub fn read_byte(&mut self, addr: u16) -> u8 {
        self.memory_bus.cpu_read(addr)
    }

    pub fn read_word(&mut self, addr: u16) -> u16 {
        let lo = self.read_byte(addr) as u16;
        let hi = self.read_byte(addr + 1) as u16;
        (hi << 8) | lo
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
        self.pc = self.memory_bus.cpu_read_word(0xFFFA);
    }

    pub fn get_cycle_count(&self, opcode: u8) -> u16 {
        match opcode {
            0x00 => 7,
            0x01 => 6,
            0x05 => 3,
            0x06 => 5,
            0x08 => 3,
            0x09 => 2,
            0x0A => 2,
            0x0D => 4,
            0x0E => 6,
            0x10 => 2,
            0x11 => 5,
            0x15 => 4,
            0x16 => 6,
            0x18 => 2,
            0x19 => 4,
            0x1D => 4,
            0x1E => 7,
            0x20 => 6,
            0x21 => 6,
            0x24 => 3,
            0x25 => 3,
            0x26 => 5,
            0x28 => 4,
            0x29 => 2,
            0x2A => 2,
            0x2C => 4,
            0x2D => 4,
            0x2E => 6,
            0x30 => 2,
            0x31 => 5,
            0x35 => 4,
            0x36 => 6,
            0x38 => 2,
            0x39 => 4,
            0x3D => 4,
            0x3E => 7,
            0x40 => 6,
            0x41 => 6,
            0x45 => 3,
            0x46 => 5,
            0x48 => 3,
            0x49 => 2,
            0x4A => 2,
            0x4C => 3,
            0x4D => 4,
            0x4E => 6,
            0x50 => 2,
            0x51 => 5,
            0x55 => 4,
            0x56 => 6,
            0x58 => 2,
            0x59 => 4,
            0x5D => 4,
            0x5E => 7,
            0x60 => 6,
            0x61 => 6,
            0x65 => 3,
            0x66 => 5,
            0x68 => 4,
            0x69 => 2,
            0x6A => 2,
            0x6C => 5,
            0x6D => 4,
            0x6E => 6,
            0x70 => 2,
            0x71 => 5,
            0x75 => 4,
            0x76 => 6,
            0x78 => 2,
            0x79 => 4,
            0x7D => 4,
            0x7E => 7,
            0x81 => 6,
            0x84 => 3,
            0x85 => 3,
            0x86 => 3,
            0x88 => 2,
            0x8A => 2,
            0x8C => 4,
            0x8D => 4,
            0x8E => 4,
            0x90 => 2,
            0x91 => 6,
            0x94 => 4,
            0x95 => 4,
            0x96 => 4,
            0x98 => 2,
            0x99 => 5,
            0x9A => 2,
            0x9D => 5,
            0xA0 => 2,
            0xA1 => 6,
            0xA2 => 2,
            0xA4 => 3,
            0xA5 => 3,
            0xA6 => 3,
            0xA8 => 2,
            0xA9 => 2,
            0xAA => 2,
            0xAC => 4,
            0xAD => 4,
            0xAE => 4,
            0xB0 => 2,
            0xB1 => 5,
            0xB4 => 4,
            0xB5 => 4,
            0xB6 => 4,
            0xB8 => 2,
            0xB9 => 4,
            0xBA => 2,
            0xBC => 4,
            0xBD => 4,
            0xBE => 4,
            0xC0 => 2,
            0xC1 => 6,
            0xC4 => 3,
            0xC5 => 3,
            0xC6 => 5,
            0xC8 => 2,
            0xC9 => 2,
            0xCA => 2,
            0xCC => 4,
            0xCD => 4,
            0xCE => 6,
            0xD0 => 2,
            0xD1 => 5,
            0xD5 => 4,
            0xD6 => 6,
            0xD8 => 2,
            0xD9 => 4,
            0xDD => 4,
            0xDE => 7,
            0xE0 => 2,
            0xE1 => 6,
            0xE4 => 3,
            0xE5 => 3,
            0xE6 => 5,
            0xE8 => 2,
            0xE9 => 2,
            0xEA => 2,
            0xEC => 4,
            0xED => 4,
            0xEE => 6,
            0xF0 => 2,
            0xF1 => 5,
            0xF5 => 4,
            0xF6 => 6,
            0xF8 => 2,
            0xF9 => 4,
            0xFD => 4,
            0xFE => 7,
            _ => 0,
        }
    }

    pub fn run(&mut self) {
        let opcode = self.fetch_byte();
        let _ = self.cycles.wrapping_add(self.get_cycle_count(opcode));
        println!(
            "Opcode: {:#02X} PC: {:#04X} A: {:#02X} X: {:#02X} Y: {:#02X}",
            opcode, self.pc, self.a, self.x, self.y
        );

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
            0x0A => self.asl(AddressMode::Accumulator),
            0x06 => self.asl(AddressMode::ZeroPage),
            0x16 => self.asl(AddressMode::ZeroPageX),
            0x0E => self.asl(AddressMode::Absolute),
            0x1E => self.asl(AddressMode::AbsoluteX),

            // BCC, BCS, BEQ, BVC, BVS
            0x90 => self.bcc(AddressMode::Relative),
            0xB0 => self.bcs(AddressMode::Relative),
            0xF0 => self.beq(AddressMode::Relative),
            0x30 => self.bmi(AddressMode::Relative),
            0xD0 => self.bne(AddressMode::Relative),
            0x10 => self.bpl(AddressMode::Relative),
            0x50 => self.bvc(AddressMode::Relative),
            0x70 => self.bvs(AddressMode::Relative),

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
            0x4A => self.lsr(AddressMode::Accumulator),
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
            0x6A => self.ror(AddressMode::Accumulator),
            0x66 => self.ror(AddressMode::ZeroPage),
            0x76 => self.ror(AddressMode::ZeroPageX),
            0x6E => self.ror(AddressMode::Absolute),
            0x7E => self.ror(AddressMode::AbsoluteX),
            0x2A => self.rol(AddressMode::Accumulator),
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
}
