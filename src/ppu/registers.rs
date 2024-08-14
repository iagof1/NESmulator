use crate::ppu::addr::AddrReg;
use crate::ppu::ctrl::CtrlReg;
use crate::ppu::mask::MaskReg;
use crate::ppu::scroll::ScrollReg;
use crate::ppu::status::StatusReg;

pub struct Registers {
    pub ctrl: CtrlReg,
    pub mask: MaskReg,
    pub status: StatusReg,
    pub addr: AddrReg,
    pub scroll: ScrollReg,
    pub nmi_interrupt: Option<u8>,
    pub internal_data_buf: u8,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            ctrl: CtrlReg::new(),
            mask: MaskReg::new(),
            status: StatusReg::new(),
            addr: AddrReg::new(),
            scroll: ScrollReg::new(),
            internal_data_buf: 0,
            nmi_interrupt: None,
        }
    }

    pub fn write_to_ppu_addr(&mut self, value: u8) {
        self.addr.update(value);
    }

    pub fn write_to_scroll(&mut self, value: u8) {
        self.scroll.write(value);
    }

    pub fn write_to_mask(&mut self, value: u8) {
        self.mask.update(value);
    }

    pub fn write_control(&mut self, value: u8) {
        let before_nmi_status = self.ctrl.generate_vblank_nmi();
        self.ctrl.update(value);
        if !before_nmi_status && self.ctrl.generate_vblank_nmi() && self.status.is_in_vblank() {
            self.nmi_interrupt = Some(1);
        }
    }

    pub fn read_status(&mut self) -> u8 {
        let data = self.status.snapshot();
        self.status.reset_vblank_status();
        self.addr.reset_latch();
        self.scroll.reset_latch();
        data
    }
}
