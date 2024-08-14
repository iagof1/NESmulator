use bitflags::bitflags;

bitflags! {
    pub struct MaskReg: u8 {
        const GREYSCALE               = 0b00000001;
        const LEFTMOST_8PXL_BACKGROUND  = 0b00000010;
        const LEFTMOST_8PXL_SPRITE      = 0b00000100;
        const SHOW_BACKGROUND         = 0b00001000;
        const SHOW_SPRITES            = 0b00010000;
        const EMPHASISE_RED           = 0b00100000;
        const EMPHASISE_GREEN         = 0b01000000;
        const EMPHASISE_BLUE          = 0b10000000;
    }
}

pub enum Color {
    Red,
    Green,
    Blue,
}

impl MaskReg {
    pub fn new() -> Self {
        MaskReg::from_bits_truncate(0b00000000)
    }

    pub fn is_grayscale(&self) -> bool {
        self.contains(MaskReg::GREYSCALE)
    }

    pub fn leftmost_8pxl_background(&self) -> bool {
        self.contains(MaskReg::LEFTMOST_8PXL_BACKGROUND)
    }

    pub fn leftmost_8pxl_sprite(&self) -> bool {
        self.contains(MaskReg::LEFTMOST_8PXL_SPRITE)
    }

    pub fn show_background(&self) -> bool {
        self.contains(MaskReg::SHOW_BACKGROUND)
    }

    pub fn show_sprites(&self) -> bool {
        self.contains(MaskReg::SHOW_SPRITES)
    }

    pub fn emphasise(&self) -> Vec<Color> {
        let mut result = Vec::<Color>::new();
        if self.contains(MaskReg::EMPHASISE_RED) {
            result.push(Color::Red);
        }
        if self.contains(MaskReg::EMPHASISE_BLUE) {
            result.push(Color::Blue);
        }
        if self.contains(MaskReg::EMPHASISE_GREEN) {
            result.push(Color::Green);
        }

        result
    }

    pub fn update(&mut self, data: u8) {
        self.bits = data;
    }
}
