use crate::ppu::addr::AddrReg;
use crate::ppu::ctrl::CtrlReg;
use crate::ppu::mask::MaskReg;
use crate::ppu::status::StatusReg;

pub struct Registers {
    pub ctrl: CtrlReg,
    pub mask: MaskReg,
    pub status: StatusReg,
    pub addr: AddrReg,
    pub internal_data_buf: u8,
    latch: bool,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            ctrl: CtrlReg::default(),
            mask: MaskReg::default(),
            status: StatusReg::default(),
            addr: AddrReg::default(),
            internal_data_buf: 0,
            latch: false,
        }
    }

    fn write_addr(&mut self, value: u8) {
        if self.latch {
            self.addr.set_high_byte(value);
        } else {
            self.addr.set_low_byte(value);
        }
        self.latch = !self.latch;
    }

    fn write_mask(&mut self, value: u8) {
        self.mask = MaskReg(value);
    }

    fn write_control(&mut self, value: u8) {
        self.ctrl = CtrlReg(value);
        self.addr.set_nametable_x(self.ctrl.nametable_x());
        self.addr.set_nametable_y(self.ctrl.nametable_y());
    }

    fn read_status(&mut self) -> u8 {
        let status = self.status.get();
        self.status.set_vertical_blank(false);
        self.latch = false;
        status
    }

    pub fn read_register(&mut self, addr: u16) -> u8 {
        println!("PPU: Reading register from address: {:#X}", addr);
        let result = match addr {
            0x2000 => self.internal_data_buf,
            0x2001 => self.internal_data_buf,
            0x2002 => self.read_status() | (self.internal_data_buf & 0x1F),
            0x2003 => self.internal_data_buf,
            0x2007 => self.internal_data_buf,
            _ => 0,
        };
        self.internal_data_buf = result;
        result
    }

    pub fn write_register(&mut self, addr: u16, value: u8) {
        println!("PPU: Writing register to address: {:#X}", addr);
        match addr {
            0x2000 => self.write_control(value),
            0x2001 => self.write_mask(value),
            0x2006 => self.write_addr(value),
            0x2007 => self.internal_data_buf = value,
            _ => {}
        }
    }
}
