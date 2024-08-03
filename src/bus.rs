use crate::nes_file::Mirroring;
use crate::ppu::PPU;

pub struct MemoryBus {
    cpu_ram: [u8; 0x800],
    pub ppu: PPU,
    prg_rom: Vec<u8>,
}

impl MemoryBus {
    pub fn new(prg_rom: Vec<u8>, chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        MemoryBus {
            cpu_ram: [0; 0x800],
            ppu: PPU::new(chr_rom, mirroring),
            prg_rom,
        }
    }

    pub fn cpu_read(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.cpu_ram[(addr & 0x07FF) as usize], // Mirroring
            0x2000..=0x3FFF => self.ppu.read_register((addr & 0x2007) as u16), // Mirrored every 8 bytes
            0x8000..=0xBFFF => self.prg_rom[(addr - 0x8000) as usize % self.prg_rom.len()],
            0xC000..=0xFFFF => self.prg_rom[(addr - 0xC000) as usize % self.prg_rom.len()],
            _ => 0,
        }
    }

    pub fn cpu_write(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x1FFF => self.cpu_ram[(addr & 0x07FF) as usize] = data, // Mirroring
            0x2000..=0x3FFF => self.ppu.write_register((addr & 0x2007) as u16, data), // Mirrored every 8 bytes
            0x8000..=0xFFFF => (), // ROM, no writes
            _ => (),
        }
    }

    pub fn cpu_read_word(&mut self, addr: u16) -> u16 {
        let lo = self.cpu_read(addr) as u16;
        let hi = self.cpu_read(addr + 1) as u16;
        (hi << 8) | lo
    }
}
