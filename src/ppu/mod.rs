mod addr;
mod ctrl;
mod frame;
mod mask;
mod registers;
mod status;

use crate::ppu::frame::Frame;
use crate::rom::Mirroring;
use crate::tile::SystemPalette;

use registers::Registers;

pub const VRAM_START: u16 = 0x2000;
pub const VRAM_END_MIRROR: u16 = 0x2FFF;
pub const VRAM_END: u16 = 0x3FFF;
pub const NAMETABLE_SIZE: u16 = 0x400;

pub struct PPU {
    pub pattern_tables: [u8; 0x2000], // Pattern tables (0x0000 - 0x1FFF)
    pub name_tables: [u8; 0x0800],    // Name tables (0x2000 - 0x2FFF, with mirroring)
    pub palette_ram: [u8; 0x20],      // Palette RAM indices (0x3F00 - 0x3F1F)
    pub oam_data: [u8; 0x100],        // Object Attribute Memory (OAM)
    pub palette: SystemPalette,
    pub chr_rom: Vec<u8>,
    pub mirroring: Mirroring,
    pub registers: Registers,
    pub frame: Frame,
    internal_data_buf: u8,
}

impl PPU {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> PPU {
        let mut ppu = PPU {
            mirroring,
            pattern_tables: [0; 0x2000],
            name_tables: [0; 0x0800],
            palette_ram: [0; 0x20],
            oam_data: [0; 256],
            chr_rom: chr_rom.clone(),
            registers: Registers::new(),
            palette: SystemPalette::new(),
            frame: Frame::new(),
            internal_data_buf: 0,
        };

        ppu.load_vram();
        ppu
    }

    pub fn load_vram(&mut self) {
        let chr_size = self.chr_rom.len();
        if chr_size >= 0x2000 {
            self.pattern_tables
                .copy_from_slice(&self.chr_rom[0..0x2000]);
        } else {
            self.pattern_tables[..chr_size].copy_from_slice(&self.chr_rom);
        }

        for i in 0..self.name_tables.len() {
            self.name_tables[i] = i as u8;
        }

        for i in 0..self.palette_ram.len() {
            self.palette_ram[i] = (i % 64) as u8;
        }
    }

    pub fn mirror_vram_addr(&self, addr: u16) -> u16 {
        let mirrored_vram = addr & VRAM_END_MIRROR;
        let vram_index = mirrored_vram - VRAM_START;
        let name_table = vram_index / NAMETABLE_SIZE;

        match (&self.mirroring, name_table) {
            (Mirroring::Vertical, 2) | (Mirroring::Vertical, 3) => {
                vram_index - (NAMETABLE_SIZE * 2)
            }
            (Mirroring::Horizontal, 1) => vram_index - NAMETABLE_SIZE,
            (Mirroring::Horizontal, 2) => vram_index - NAMETABLE_SIZE,
            (Mirroring::Horizontal, 3) => vram_index - (NAMETABLE_SIZE * 2),
            _ => vram_index,
        }
    }

    pub fn read_data(&mut self) -> u8 {
        let addr = self.registers.addr.address();
        self.registers
            .addr
            .increment(self.registers.ctrl.increment_amount());
        println!("PPU: Reading data from PPU address: {:#X}", addr);
        match addr {
            0x0000..=0x1FFF => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.pattern_tables[addr as usize];
                result
            }
            0x2000..=VRAM_END_MIRROR => {
                let result = self.internal_data_buf;
                let mirrored_addr = self.mirror_vram_addr(addr);
                self.internal_data_buf = self.name_tables[mirrored_addr as usize];
                result
            }
            0x3F00..=VRAM_END => self.palette_ram[self.mirror_vram_addr(addr) as usize],
            _ => unimplemented!(),
        }
    }

    pub fn write_data(&mut self, value: u8) {
        let addr = self.registers.addr.address();
        println!("PPU: Writing data to PPU address: {:#X}", addr);
        match addr {
            0x0000..=0x1FFF => self.pattern_tables[addr as usize] = value,
            0x2000..=VRAM_END_MIRROR => {
                let mirrored_addr = self.mirror_vram_addr(addr);
                self.name_tables[mirrored_addr as usize] = value;
            }
            0x3F00..=VRAM_END => self.palette_ram[self.mirror_vram_addr(addr) as usize] = value,
            _ => unimplemented!(),
        }
        self.registers
            .addr
            .increment(self.registers.ctrl.increment_amount());
    }

    pub fn render_frame(&mut self) -> Vec<u8> {
        self.frame.render_frame(
            &self.pattern_tables,
            &self.name_tables,
            &self.palette_ram,
            &self.palette,
        )
    }
}
