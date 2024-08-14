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
    pub oam_data: [u8; 0x100],

    pub chr_rom: Vec<u8>,
    pub mirroring: Mirroring,
    pub registers: Registers,

    scanline: u16,
    cycles: usize,
}

impl PPU {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> PPU {
        PPU {
            vram: [0; 0x800],
            mirroring,
            palette_table: [0; 32],
            oam_data: [0; 256],
            chr_rom: chr_rom,
            registers: Registers::new(),

            cycles: 0,
            scanline: 0,
        }
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

    pub fn read_data(&mut self) -> u8 {
        let addr = self.registers.addr.get();
        self.registers
            .addr
            .increment(self.registers.ctrl.vram_addr_increment());
        println!("PPU: Reading data from addr: {:#X}", addr);
        match addr {
            0x0000..=0x1FFF => {
                let result = self.registers.internal_data_buf;
                self.registers.internal_data_buf = self.chr_rom[addr as usize];
                result
            }
            0x2000..=0x2FFF => {
                let result = self.registers.internal_data_buf;
                let mirrored_addr = self.mirror_vram_addr(addr);
                self.registers.internal_data_buf = self.vram[mirrored_addr as usize];
                result
            }
            0x3000..=0x3EFF => unimplemented!(),
            0x3F10 | 0x3F14 | 0x3F18 | 0x3F1C => {
                let mirrored_addr = addr - 0x10;
                self.palette_table[(mirrored_addr - 0x3F00) as usize]
            }
            0x3F00..=0x3FFF => self.palette_table[(addr - 0x3F00) as usize],
            _ => panic!("Unimplemented PPU read at address: {:#X}", addr),
        }
    }

    pub fn write_to_data(&mut self, value: u8) {
        let addr = self.registers.addr.get();
        println!("PPU: Writing data to addr: {:#X}", addr);
        match addr {
            0x0000..=0x1FFF => println!("Attempt to write to chr rom space: {}", addr),
            0x2000..=0x2FFF => {
                let mirrored_addr = self.mirror_vram_addr(addr);
                self.vram[mirrored_addr as usize] = value;
            }
            0x3000..=0x3EFF => unimplemented!(),
            0x3F10 | 0x3F14 | 0x3F18 | 0x3F1C => {
                let addr_mirror = addr - 0x10;
                self.palette_table[(addr_mirror - 0x3F00) as usize] = value;
            }
            0x3F00..=0x3FFF => {
                self.palette_table[(addr - 0x3F00) as usize] = value;
            }
            _ => panic!("Unexpected write to mirrored space: {}", addr),
        }
        self.registers
            .addr
            .increment(self.registers.ctrl.vram_addr_increment());
    }

    pub fn tick(&mut self, cycles: u8) -> bool {
        self.cycles += cycles as usize;
        if self.cycles >= 341 {
            if self.is_sprite_0_hit(self.cycles) {
                self.registers.status.set_sprite_zero_hit(true);
            }

            self.cycles = self.cycles - 341;
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
                return true;
            }
        }
        return false;
    }
    pub fn poll_nmi_interrupt(&mut self) -> Option<u8> {
        self.registers.nmi_interrupt.take()
    }

    fn is_sprite_0_hit(&self, cycle: usize) -> bool {
        let y = self.oam_data[0] as usize;
        let x = self.oam_data[3] as usize;
        (y == self.scanline as usize) && x <= cycle && self.registers.mask.show_sprites()
    }
}
