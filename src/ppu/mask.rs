use bitfield::bitfield;

bitfield! {
    pub struct MaskReg(u8);
    impl Debug;
    pub grayscale, set_grayscale: 0;
    pub show_bg_leftmost, set_show_bg_leftmost: 1;
    pub show_sprites_leftmost, set_show_sprites_leftmost: 2;
    pub show_bg, set_show_bg: 3;
    pub show_sprites, set_show_sprites: 4;
    pub emphasize_red, set_emphasize_red: 5;
    pub emphasize_green, set_emphasize_green: 6;
    pub emphasize_blue, set_emphasize_blue: 7;
}

impl Default for MaskReg {
    fn default() -> Self {
        MaskReg(0)
    }
}
