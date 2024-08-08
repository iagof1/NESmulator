use bitfield::bitfield;

bitfield! {
    pub struct AddrReg(u16);
    impl Debug;
    pub coarse_x, set_coarse_x: 4, 0;
    pub coarse_y, set_coarse_y: 9, 5;
    pub nametable_x, set_nametable_x: 10;
    pub nametable_y, set_nametable_y: 11;
    pub fine_y, set_fine_y: 14, 12;
    pub u16, address, _: 13,  0;
    pub u8, high_byte, set_high_byte: 13, 8;
    pub u8, low_byte, set_low_byte: 7, 0;
    pub u16, get, _: 14,  0;
}

impl Default for AddrReg {
    fn default() -> Self {
        AddrReg(0)
    }
}

impl AddrReg {
    pub fn increment(&mut self, amount: u16) {
        self.0 = self.0.wrapping_add(amount);
    }
}
