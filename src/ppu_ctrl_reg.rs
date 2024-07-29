use bitflags::bitflags;

bitflags! {
    pub struct CtrlRegister: u8 {
        const NAMETABLE1    = 0b00000001;
        const NAMETABLE2    = 0b00000010;
        const VRAM_INC      = 0b00000100;
        const SPRITE_TABLE  = 0b00001000;
        const BG_TABLE      = 0b00010000;
        const SPRITE_SIZE   = 0b00100000;
        const MASTER_SLAVE  = 0b01000000;
        const NMI           = 0b10000000;
    }
}

impl CtrlRegister {
    pub fn new() -> Self {
        CtrlRegister::from_bits_truncate(0b00000000)
    }

    pub fn vram_addr_increment(&self) -> u8 {
        if !self.contains(CtrlRegister::VRAM_INC) {
            1
        } else {
            32
        }
    }

    pub fn update(&mut self, data: u8) {
        self.bits = data;
    }
}
