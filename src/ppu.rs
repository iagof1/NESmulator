use crate::nes_file::Mirroring;
use crate::ppu_addr_reg::AddrRegister;
use crate::ppu_ctrl_reg::CtrlRegister;
use crate::tile::{SystemPalette, Tile};

pub const VRAM_START: u16 = 0x2000;
pub const VRAM_END_MIRROR: u16 = 0x2FFF;
pub const VRAM_END: u16 = 0x3FFF;
pub const NAMETABLE_SIZE: u16 = 0x400;

pub struct PPU {
    pub pattern_tables: [u8; 0x2000], // Pattern tables (0x0000 - 0x1FFF)
    pub name_tables: [u8; 0x0800],    // Name tables (0x2000 - 0x2FFF, with mirroring)
    pub palette_ram: [u8; 0x20],      // Palette RAM indices (0x3F00 - 0x3F1F)
    pub oam_data: [u8; 256],          // Object Attribute Memory (OAM)
    pub palette: SystemPalette,
    pub chr_rom: Vec<u8>,
    pub mirroring: Mirroring,
    pub addr: AddrRegister,
    pub ctrl: CtrlRegister,
    internal_data_buf: u8,
}

impl PPU {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> PPU {
        let mut ppu = PPU {
            pattern_tables: [0; 0x2000],
            name_tables: [0; 0x0800],
            palette_ram: [0; 0x20],
            oam_data: [0; 256],
            chr_rom: chr_rom.clone(),
            mirroring,
            palette: SystemPalette::new(),
            addr: AddrRegister::new(),
            ctrl: CtrlRegister::new(),
            internal_data_buf: 0,
        };

        ppu.load_vram();

        ppu
    }

    pub fn set_addr(&mut self, addr: u8) {
        self.addr.update(addr);
    }

    pub fn set_ctrl(&mut self, value: u8) {
        self.ctrl.update(value);
    }

    pub fn read_register(&mut self, addr: u16) -> u8 {
        match addr {
            0x2000 => self.ctrl.bits(),
            0x2002 => self.read_status(),
            0x2007 => self.read_data(),
            _ => 0,
        }
    }

    pub fn write_register(&mut self, addr: u16, value: u8) {
        match addr {
            0x2000 => self.set_ctrl(value),
            0x2006 => self.set_addr(value),
            0x2007 => self.write_data(value),
            _ => {}
        }
    }

    pub fn read_status(&self) -> u8 {
        // Implement the status register read logic
        0
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
        let addr = self.addr.get();
        self.addr.increment(self.ctrl.vram_addr_increment());

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
        let addr = self.addr.get();
        match addr {
            0x0000..=0x1FFF => self.pattern_tables[addr as usize] = value,
            0x2000..=VRAM_END_MIRROR => {
                let mirrored_addr = self.mirror_vram_addr(addr);
                self.name_tables[mirrored_addr as usize] = value;
            }
            0x3F00..=VRAM_END => self.palette_ram[self.mirror_vram_addr(addr) as usize] = value,
            _ => unimplemented!(),
        }
        self.addr.increment(self.ctrl.vram_addr_increment());
    }

    pub fn render_frame(&self) -> Vec<u8> {
        let mut frame = vec![0u32; 256 * 240];
        for tile_y in 0..30 {
            for tile_x in 0..32 {
                let index = tile_y * 32 + tile_x;
                if index >= self.pattern_tables.len() / 16 {
                    continue;
                }

                let tile = self.get_tile(index);
                let palette = self.get_tile_palette(tile_x, tile_y);

                for (row, pixel_row) in tile.pixels.iter().enumerate() {
                    for (col, &pixel) in pixel_row.iter().enumerate() {
                        let color = palette[pixel as usize];
                        let x = tile_x * 8 + col;
                        let y = tile_y * 8 + row;

                        if x < 256 && y < 240 {
                            frame[y * 256 + x] = color;
                        } else {
                            println!("Out of bounds: x = {}, y = {}", x, y);
                        }
                    }
                }
            }
        }
        let mut frame_u8 = Vec::with_capacity(frame.len() * 4);
        for pixel in frame {
            frame_u8.extend_from_slice(&pixel.to_le_bytes());
        }

        frame_u8
    }

    fn get_tile(&self, index: usize) -> Tile {
        let mut pixels = [[0u8; 8]; 8];
        let base = index * 16;

        for row in 0..8 {
            let low_byte = self.pattern_tables[base + row];
            let high_byte = self.pattern_tables[base + row + 8];

            for col in 0..8 {
                let low_bit = (low_byte >> (7 - col)) & 1;
                let high_bit = (high_byte >> (7 - col)) & 1;
                pixels[row][col] = (high_bit << 1) | low_bit;
            }
        }

        Tile { pixels }
    }

    fn get_tile_palette(&self, tile_x: usize, tile_y: usize) -> [u32; 4] {
        let attr_x = tile_x / 4;
        let attr_y = tile_y / 4;
        let attr_byte = self.name_tables[0x3C0 + attr_y * 8 + attr_x];

        let shift = match (tile_x % 4 / 2, tile_y % 4 / 2) {
            (0, 0) => 0,
            (1, 0) => 2,
            (0, 1) => 4,
            (1, 1) => 6,
            _ => 0,
        };

        let palette_index = (attr_byte >> shift) & 0b11;
        let frame_palette = &self.palette_ram[palette_index as usize * 4..];

        [
            self.palette.colors[frame_palette[0] as usize],
            self.palette.colors[frame_palette[1] as usize],
            self.palette.colors[frame_palette[2] as usize],
            self.palette.colors[frame_palette[3] as usize],
        ]
    }
}
