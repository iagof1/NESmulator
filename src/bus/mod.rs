use crate::joypad::Joypad;
use crate::ppu::PPU;
use crate::rom::Rom;

const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;

pub struct Bus<'call> {
    pub ram: [u8; 0x800],
    pub ppu: PPU,
    pub joypad1: Joypad,
    prg_rom: Vec<u8>,
    cycles: usize,
    gameloop_callback: Box<dyn FnMut(&PPU, &mut Joypad) + 'call>,
}

impl<'a> Bus<'a> {
    pub fn new<'call, F>(rom: Rom, gameloop_callback: F) -> Bus<'call>
    where
        F: FnMut(&PPU, &mut Joypad) + 'call,
    {
        let ppu = PPU::new(rom.chr_rom, rom.mirroring);

        Bus {
            ram: [0; 0x800],
            ppu: ppu,
            joypad1: Joypad::new(),
            prg_rom: rom.prg_rom,
            cycles: 0,
            gameloop_callback: Box::from(gameloop_callback),
        }
    }

    fn read_prg_rom(&self, addr: u16) -> u8 {
        let len = self.prg_rom.len();
        if len == 0 {
            return 0;
        }
        let prg_rom_addr = (addr - 0x8000) as usize % len;
        self.prg_rom[prg_rom_addr]
    }

    pub fn mem_read(&mut self, addr: u16) -> u8 {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00000111_11111111;
                self.ram[mirror_down_addr as usize]
            }
            0x2000 | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 => 0,
            0x2002 => self.ppu.registers.read_status(),
            0x2004 => self.ppu.registers.read_oam_data(),
            0x2007 => self.ppu.read_data(),

            0x4000..=0x4015 => {
                //ignore APU
                0
            }

            0x4016 => self.joypad1.read(),

            0x4017 => 0,
            0x2008..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00100000_00000111;
                self.mem_read(mirror_down_addr)
            }
            0x8000..=0xFFFF => self.read_prg_rom(addr),

            _ => {
                // println!("Ignoring mem access at {:x}", addr);
                0
            }
        }
    }

    pub fn mem_write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b11111111111;
                self.ram[mirror_down_addr as usize] = data;
            }
            0x2000 => {
                self.ppu.registers.write_control(data);
            }
            0x2001 => {
                self.ppu.registers.write_to_mask(data);
            }

            0x2002 => panic!("attempt to write to PPU status register"),

            0x2003 => {
                self.ppu.registers.write_to_oam_addr(data);
            }
            0x2004 => {
                self.ppu.registers.write_to_oam_data(data);
            }
            0x2005 => {
                self.ppu.registers.write_to_scroll(data);
            }

            0x2006 => {
                self.ppu.registers.write_to_ppu_addr(data);
            }
            0x2007 => {
                self.ppu.write_to_data(data);
            }
            0x4000..=0x4013 | 0x4015 => {
                //ignore APU
            }

            0x4016 => {
                self.joypad1.write(data);
            }

            0x4017 => {
                // ignore joypad 2
            }

            0x4014 => {
                let mut buffer: [u8; 0x100] = [0; 0x100];
                let hi: u16 = (data as u16) << 8;
                for i in 0..256u16 {
                    buffer[i as usize] = self.mem_read(hi + i)
                }
                self.ppu.write_oam_dma(&buffer);
            }

            0x2008..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00100000_00000111;
                self.mem_write(mirror_down_addr, data);
            }
            0x8000..=0xFFFF => panic!("Attempt to write to Cartridge ROM space: {:x}", addr),

            _ => {}
        }
    }

    pub fn read_word(&mut self, addr: u16) -> u16 {
        let lo = self.mem_read(addr) as u16;
        let hi = self.mem_read(addr + 1) as u16;
        (hi << 8) | lo
    }

    pub fn tick(&mut self, cycles: u8) {
        self.cycles += cycles as usize;

        let in_vblank_before = self.ppu.registers.status.is_in_vblank();
        self.ppu.tick(cycles * 3);
        let in_vblank_after = self.ppu.registers.status.is_in_vblank();

        if !in_vblank_before && in_vblank_after {
            (self.gameloop_callback)(&self.ppu, &mut self.joypad1);
        }
    }
}
