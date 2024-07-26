struct PPU {
    pub vram: [u8; 2048],
    pub oam_data: [u8; 256],
    pub palette: [u8; 32],
    pub chr_rom: Vec<u8>,
    pub mirroring: Mirroring,
}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            vram: [0; 2048],
            oam_data: [0; 256],
            palette: [0; 32],
            chr_rom: Vec::new(),
            mirroring: Mirroring::Horizontal,
        }
    }

    pub fn load_chr_rom(&mut self, chr_rom: Vec<u8>) {
        self.chr_rom = chr_rom;
    }
}