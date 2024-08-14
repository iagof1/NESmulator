use crate::ppu::PPU;
use crate::rom::Rom;

const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;

pub struct Bus<'call> {
    pub ram: [u8; 0x800],
    pub ppu: PPU,
    prg_rom: Vec<u8>,
    cycles: usize,
    gameloop_callback: Box<dyn FnMut(&PPU) + 'call>,
}

impl<'a> Bus<'a> {
    pub fn new<'call, F>(rom: Rom, gameloop_callback: F) -> Bus<'call>
    where
        F: FnMut(&PPU) + 'call,
    {
        let ppu = PPU::new(rom.chr_rom, rom.mirroring);

        Bus {
            ram: [0; 0x800],
            ppu: ppu,
            prg_rom: rom.prg_rom,
            cycles: 0,
            gameloop_callback: Box::from(gameloop_callback),
        }
    }

    fn read_prg_rom(&self, addr: u16) -> u8 {
        let mut prg_rom_addr = addr - 0x8000;
        if self.prg_rom.len() == 0x4000 && prg_rom_addr >= 0x4000 {
            prg_rom_addr = prg_rom_addr % 0x4000;
        }
        self.prg_rom[prg_rom_addr as usize]
    }

    pub fn mem_read(&mut self, addr: u16) -> u8 {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00000111_11111111;
                self.ram[mirror_down_addr as usize]
            }
            0x2000 | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 => {
                // panic!("Attempt to read from write-only PPU address {:x}", addr);
                0
            }
            0x2002 => self.ppu.registers.read_status(),
            // 0x2004 => self.ppu.registers.read_oam_data(),
            0x2007 => self.ppu.read_data(),

            0x4000..=0x4015 => {
                //ignore APU
                0
            }

            0x4016 => {
                // self.joypad1.read()
                0
            }

            0x4017 => {
                // ignore joypad 2
                0
            }
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
                // self.ppu.registers.write_to_oam_addr(data);
            }
            0x2004 => {
                // self.ppu.registers.write_to_oam_data(data);
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
                // ignore joypad 1
            }

            0x4017 => {
                // ignore joypad 2
            }

            // https://wiki.nesdev.com/w/index.php/PPU_programmer_reference#OAM_DMA_.28.244014.29_.3E_write
            0x4014 => {
                // let mut buffer: [u8; 256] = [0; 256];
                // let hi: u16 = (data as u16) << 8;
                // for i in 0..256u16 {
                //     buffer[i as usize] = self.mem_read(hi + i);
                // }

                // self.ppu.write_oam_dma(&buffer);

                // todo: handle this eventually
                // let add_cycles: u16 = if self.cycles % 2 == 1 { 514 } else { 513 };
                // self.tick(add_cycles); //todo this will cause weird effects as PPU will have 513/514 * 3 ticks
            }

            0x2008..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_addr = addr & 0b00100000_00000111;
                self.mem_write(mirror_down_addr, data);
                // todo!("PPU is not supported yet");
            }
            0x8000..=0xFFFF => panic!("Attempt to write to Cartridge ROM space: {:x}", addr),

            _ => {
                println!("Ignoring mem write-access at {:x}", addr);
            }
        }
    }

    pub fn read_word(&mut self, addr: u16) -> u16 {
        let lo = self.mem_read(addr) as u16;
        let hi = self.mem_read(addr + 1) as u16;
        (hi << 8) | lo
    }

    pub fn tick(&mut self, cycles: u8) {
        self.cycles += cycles as usize;

        let nmi_before = self.ppu.registers.nmi_interrupt.is_some();
        self.ppu.tick(cycles * 3);
        let nmi_after = self.ppu.registers.nmi_interrupt.is_some();

        if !nmi_before && nmi_after {
            (self.gameloop_callback)(&self.ppu);
        }
    }
}
