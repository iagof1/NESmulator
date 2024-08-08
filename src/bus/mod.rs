use crate::ppu::PPU;
use crate::rom::Mirroring;

pub struct Bus {
    pub ram: [u8; 0x800],
    pub ppu: PPU,
    prg_rom: Vec<u8>,
}

impl Bus {
    pub fn new(prg_rom: Vec<u8>, chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        Bus {
            ram: [0; 0x800],
            ppu: PPU::new(chr_rom, mirroring),
            prg_rom,
        }
    }

    pub fn unclocked_read_byte(&mut self, addr: u16) -> u8 {
        println!("Bus: Read from address {:04X}", addr);
        match addr {
            0x0000..=0x1FFF => self.ram[addr as usize % 0x0800],
            0x2000..=0x3FFF => self.ppu.registers.read_register((addr & 0x2007) as u16),
            0x8000..=0xFFFF => self.prg_rom[(addr - 0x8000) as usize % self.prg_rom.len()],
            _ => 0,
        }
    }

    pub fn unclocked_write_byte(&mut self, addr: u16, data: u8) {
        println!("Bus: Write to address {:04X} with data {:02X}", addr, data);
        match addr {
            0x0000..=0x1FFF => self.ram[addr as usize % 0x0800] = data,
            0x2000..=0x3FFF => self
                .ppu
                .registers
                .write_register((addr & 0x2007) as u16, data),
            0x8000..=0xFFFF => (),
            _ => (),
        }
    }

    pub fn read_word(&mut self, addr: u16) -> u16 {
        let lo = self.unclocked_read_byte(addr) as u16;
        let hi = self.unclocked_read_byte(addr + 1) as u16;
        (hi << 8) | lo
    }
}
