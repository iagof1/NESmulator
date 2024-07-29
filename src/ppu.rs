use crate::nes_file::Mirroring;
use crate::ppu_addr_reg::AddrRegister;
use crate::ppu_ctrl_reg::CtrlRegister;

pub const VRAM_START: u16 = 0x2000;
pub const VRAM_END_MIRROR: u16 = 0x2FFF;
pub const VRAM_END: u16 = 0x3FFF;
pub const NAMETABLE_SIZE: u16 = 0x400;

pub struct PPU {
    pub vram: [u8; 2048],
    pub oam_data: [u8; 256],
    pub palette: [u8; 32],
    pub chr_rom: Vec<u8>,
    pub mirroring: Mirroring,
    pub addr: AddrRegister,
    pub ctrl: CtrlRegister,
    internal_data_buf: u8,
}

impl PPU {
    pub fn new(chr_room: Vec<u8>, mirroring: Mirroring) -> PPU {
        PPU {
            vram: [0; 2048],
            oam_data: [0; 256],
            palette: [0; 32],
            chr_rom: chr_room,
            mirroring: mirroring,
            addr: AddrRegister::new(),
            ctrl: CtrlRegister::new(),
            internal_data_buf: 0,
        }
    }

    pub fn set_addr(&mut self, addr: u8) {
        self.addr.update(addr);
    }

    pub fn set_ctrl(&mut self, value: u8) {
        self.ctrl.update(value);
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

        let result = match addr {
            0..=0x1fff => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.chr_rom[addr as usize];
                result
            }
            VRAM_START..=VRAM_END_MIRROR => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.vram[self.mirror_vram_addr(addr) as usize];
                result
            }
            _ => unimplemented!(),
        };
        result
    }
}
