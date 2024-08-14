use bitflags::bitflags;

bitflags! {
    pub struct StatusReg: u8 {
        const NOTUSED          = 0b00000001;
        const NOTUSED2         = 0b00000010;
        const NOTUSED3         = 0b00000100;
        const NOTUSED4         = 0b00001000;
        const NOTUSED5         = 0b00010000;
        const SPRITE_OVERFLOW  = 0b00100000;
        const SPRITE_ZERO_HIT  = 0b01000000;
        const VBLANK_STARTED   = 0b10000000;
    }
}

impl StatusReg {
    pub fn new() -> Self {
        StatusReg::from_bits_truncate(0b00000000)
    }

    pub fn set_vblank_status(&mut self, status: bool) {
        self.set(StatusReg::VBLANK_STARTED, status);
    }

    pub fn set_sprite_zero_hit(&mut self, status: bool) {
        self.set(StatusReg::SPRITE_ZERO_HIT, status);
    }

    pub fn set_sprite_overflow(&mut self, status: bool) {
        self.set(StatusReg::SPRITE_OVERFLOW, status);
    }

    pub fn reset_vblank_status(&mut self) {
        self.remove(StatusReg::VBLANK_STARTED);
    }

    pub fn is_in_vblank(&self) -> bool {
        self.contains(StatusReg::VBLANK_STARTED)
    }

    pub fn snapshot(&self) -> u8 {
        self.bits
    }
}
