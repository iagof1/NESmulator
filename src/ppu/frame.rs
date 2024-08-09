use crate::ppu::SystemPalette;
use crate::tile::Tile;

pub struct Frame {
    pub pixels: Vec<u32>,
}

impl Frame {
    pub fn new() -> Frame {
        Frame {
            pixels: vec![0u32; 256 * 240],
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.pixels.len() {
            self.pixels[i] = 0;
        }
    }

    fn get_tile(&self, index: usize, pattern_tables: &[u8]) -> Tile {
        let mut pixels = [[0u8; 8]; 8];
        let base = index * 16;

        for row in 0..8 {
            let low_byte = pattern_tables[base + row];
            let high_byte = pattern_tables[base + row + 8];

            for col in 0..8 {
                let low_bit = (low_byte >> (7 - col)) & 1;
                let high_bit = (high_byte >> (7 - col)) & 1;
                pixels[row][col] = (high_bit << 1) | low_bit;
            }
        }

        Tile { pixels }
    }

    fn get_tile_palette(
        &self,
        tile_x: usize,
        tile_y: usize,
        name_tables: &[u8],
        palette_ram: &[u8],
        palette: &SystemPalette,
    ) -> [u32; 4] {
        let attr_x = tile_x / 4;
        let attr_y = tile_y / 4;
        let attr_byte = name_tables[0x3C0 + attr_y * 8 + attr_x];

        let shift = match (tile_x % 4 / 2, tile_y % 4 / 2) {
            (0, 0) => 0,
            (1, 0) => 2,
            (0, 1) => 4,
            (1, 1) => 6,
            _ => 0,
        };

        let palette_index = (attr_byte >> shift) & 0b11;
        let frame_palette = &palette_ram[palette_index as usize * 4..];

        [
            palette.colors[frame_palette[0] as usize],
            palette.colors[frame_palette[1] as usize],
            palette.colors[frame_palette[2] as usize],
            palette.colors[frame_palette[3] as usize],
        ]
    }

    pub fn render_frame(
        &self,
        pattern_tables: &[u8],
        name_tables: &[u8],
        palette_ram: &[u8],
        palette: &SystemPalette,
    ) -> Vec<u8> {
        let mut frame = self.pixels.clone();

        for tile_y in 0..30 {
            for tile_x in 0..32 {
                let index = tile_y * 32 + tile_x;
                if index >= pattern_tables.len() / 16 {
                    continue;
                }

                let tile = self.get_tile(index, pattern_tables);
                let palette =
                    self.get_tile_palette(tile_x, tile_y, name_tables, palette_ram, palette);

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
}
