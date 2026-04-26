mod addr;
mod ctrl;
mod mask;
mod registers;
mod scroll;
mod status;

use crate::rom::Mirroring;

use registers::Registers;

pub struct PPU {
    pub vram: [u8; 0x800],
    pub palette_table: [u8; 0x20],
    pub chr_rom: Vec<u8>,
    pub mirroring: Mirroring,
    pub registers: Registers,

    scanline: u16,
    cycles: usize,
}

impl PPU {
    pub fn new_empty_rom() -> Self {
        PPU::new(vec![0; 2048], Mirroring::Horizontal)
    }

    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> PPU {
        PPU {
            vram: [0; 0x800],
            mirroring,
            chr_rom: chr_rom,
            palette_table: [0; 0x20],
            registers: Registers::new(),

            cycles: 0,
            scanline: 0,
        }
    }

    pub fn oam_data(&self) -> &[u8; 0x100] {
        &self.registers.oam_data
    }

    pub fn mirror_vram_addr(&self, addr: u16) -> u16 {
        let mirrored_vram = addr & 0b10111111111111;
        let vram_index = mirrored_vram - 0x2000;
        let name_table = vram_index / 0x400;

        match (&self.mirroring, name_table) {
            (Mirroring::Vertical, 2) | (Mirroring::Vertical, 3) => vram_index - 0x800,
            (Mirroring::Horizontal, 1) => vram_index - 0x400,
            (Mirroring::Horizontal, 2) => vram_index - 0x400,
            (Mirroring::Horizontal, 3) => vram_index - 0x800,
            _ => vram_index,
        }
    }

    pub fn write_oam_dma(&mut self, data: &[u8; 256]) {
        for x in data.iter() {
            self.registers.oam_data[self.registers.oam_addr as usize] = *x;
            self.registers.oam_addr = self.registers.oam_addr.wrapping_add(1);
        }
    }

    fn palette_index(addr: u16) -> usize {
        let mut idx = (addr - 0x3F00) as usize & 0x1F;
        // 0x3F10/14/18/1C mirror 0x3F00/04/08/0C
        if idx == 0x10 || idx == 0x14 || idx == 0x18 || idx == 0x1C {
            idx -= 0x10;
        }
        idx
    }

    pub fn read_data(&mut self) -> u8 {
        let addr = self.registers.addr.get();
        self.registers
            .addr
            .increment(self.registers.ctrl.vram_addr_increment());
        match addr {
            0x0000..=0x1FFF => {
                let result = self.registers.internal_data_buf;
                self.registers.internal_data_buf =
                    *self.chr_rom.get(addr as usize).unwrap_or(&0);
                result
            }
            0x2000..=0x2FFF => {
                let result = self.registers.internal_data_buf;
                let mirrored_addr = self.mirror_vram_addr(addr);
                self.registers.internal_data_buf = self.vram[mirrored_addr as usize];
                result
            }
            0x3000..=0x3EFF => {
                let result = self.registers.internal_data_buf;
                let mirrored_addr = self.mirror_vram_addr(addr - 0x1000);
                self.registers.internal_data_buf = self.vram[mirrored_addr as usize];
                result
            }
            0x3F00..=0x3FFF => self.palette_table[Self::palette_index(addr)],
            _ => 0,
        }
    }

    pub fn write_to_data(&mut self, value: u8) {
        let addr = self.registers.addr.get();
        match addr {
            0x0000..=0x1FFF => {} // CHR ROM read-only
            0x2000..=0x2FFF => {
                let mirrored_addr = self.mirror_vram_addr(addr);
                self.vram[mirrored_addr as usize] = value;
            }
            0x3000..=0x3EFF => {
                let mirrored_addr = self.mirror_vram_addr(addr - 0x1000);
                self.vram[mirrored_addr as usize] = value;
            }
            0x3F00..=0x3FFF => {
                self.palette_table[Self::palette_index(addr)] = value;
            }
            _ => {}
        }
        self.registers
            .addr
            .increment(self.registers.ctrl.vram_addr_increment());
    }

    pub fn tick(&mut self, cycles: u8) -> bool {
        self.cycles += cycles as usize;
        let mut new_frame = false;
        while self.cycles >= 341 {
            if self.is_sprite_0_hit(self.cycles) {
                self.registers.status.set_sprite_zero_hit(true);
            }

            self.cycles -= 341;
            self.scanline += 1;

            if self.scanline == 241 {
                self.registers.status.set_vblank_status(true);
                self.registers.status.set_sprite_zero_hit(false);
                if self.registers.ctrl.generate_vblank_nmi() {
                    self.registers.nmi_interrupt = Some(1);
                }
            }

            if self.scanline >= 262 {
                self.scanline = 0;
                self.registers.nmi_interrupt = None;
                self.registers.status.set_sprite_zero_hit(false);
                self.registers.status.reset_vblank_status();
                new_frame = true;
            }
        }
        new_frame
    }

    pub fn scanline(&self) -> u16 {
        self.scanline
    }
    pub fn cycle(&self) -> usize {
        self.cycles
    }
    pub fn poll_nmi_interrupt(&mut self) -> Option<u8> {
        self.registers.nmi_interrupt.take()
    }

    fn is_sprite_0_hit(&self, cycle: usize) -> bool {
        let y = self.registers.oam_data[0] as usize;
        let x = self.registers.oam_data[3] as usize;
        (y == self.scanline as usize) && x <= cycle && self.registers.mask.show_sprites()
    }
}
