use bitfield::bitfield;

bitfield! {
    pub struct StatusReg(u8);
    impl Debug;
    pub sprite_overflow, set_sprite_overflow: 5;
    pub sprite_zero_hit, set_sprite_zero_hit: 6;
    pub vertical_blank, set_vertical_blank: 7;
    pub get, _: 7, 0;
}

impl Default for StatusReg {
    fn default() -> Self {
        StatusReg(0)
    }
}
