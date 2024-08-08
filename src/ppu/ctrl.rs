use bitfield::bitfield;

bitfield! {
    pub struct CtrlReg(u8);
    impl Debug;
    pub nametable_x, set_nametable_x: 0;
    pub nametable_y, set_nametable_y: 1;
    pub increment_mode, set_increment_mode: 2;
    pub pattern_sprite, set_pattern_sprite: 3;
    pub pattern_background, set_pattern_background: 4;
    pub sprite_size, set_sprite_size: 5;
    pub slave_mode, set_slave_mode: 6;
    pub enable_nmi, set_enable_nmi: 7;
}

impl Default for CtrlReg {
    fn default() -> Self {
        CtrlReg(0)
    }
}

impl CtrlReg {
    pub fn increment_amount(&self) -> u16 {
        if self.increment_mode() {
            32
        } else {
            1
        }
    }
}
