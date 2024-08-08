pub trait Mapper {
    fn signal_scanline(&mut self);
    fn read_prg_byte(&self, addr: u16) -> u8;
    fn write_prg_byte(&mut self, addr: u16, data: u8);
    fn read_chr_byte(&self, addr: u16) -> u8;
    fn write_chr_byte(&mut self, addr: u16, data: u8);
}

pub struct Mapper0 {
    prg_rom: Vec<u8>,
}

impl Mapper0 {
    pub fn new(prg_rom: Vec<u8>) -> Self {
        Self { prg_rom }
    }
}
