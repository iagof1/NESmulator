use crate::cpu::{StatusFlags, CPU};

#[derive(Debug)]
pub enum AddressMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    Accumulator,
    Relative,
}

macro_rules! logical {
    ($fn_name:ident, $op:tt) => {
        pub fn $fn_name(&mut self, mode: AddressMode) {
            let (_, value) = self.get_operand(mode);
            self.a = self.a $op value;
            self.update_zero_and_negative_flags(self.a);
        }
    };
}

macro_rules! load_register {
    ($fn_name:ident, $reg:ident) => {
        pub fn $fn_name(&mut self, mode: AddressMode) {
            let (_, value) = self.get_operand(mode);
            self.$reg = value;
            self.update_zero_and_negative_flags(self.$reg);
        }
    };
}

macro_rules! branch {
    ($fn_name:ident, $flag:ident, $condition:expr) => {
        pub fn $fn_name(&mut self, mode: AddressMode) {
            match mode {
                AddressMode::Relative => {
                    let offset = self.fetch_byte() as i8;
                    if $condition(self.status.contains(StatusFlags::$flag)) {
                        self.pc = self.pc.wrapping_add(offset as u16);
                    }
                }
                _ => panic!(
                    "Addressing mode not supported for {} instruction",
                    stringify!($fn_name)
                ),
            }
        }
    };
}

macro_rules! compare {
    ($fn_name:ident, $reg:ident) => {
        pub fn $fn_name(&mut self, mode: AddressMode) {
            let (_, value) = self.get_operand(mode);

            let result = self.$reg.wrapping_sub(value);

            self.status.set(StatusFlags::CARRY, self.$reg >= value);
            self.status.set(StatusFlags::ZERO, self.$reg == value);
            self.status.set(StatusFlags::NEGATIVE, result & 0x80 != 0);
        }
    };
}

macro_rules! rotate {
    ($fn_name:ident, $rotate_fn:ident, $carry_check:expr) => {
        pub fn $fn_name(&mut self, mode: AddressMode) {
            let (addr, value) = self.get_operand(mode);
            let result = self.$rotate_fn(value);

            if addr != 0 {
                self.memory[addr as usize] = result;
            } else {
                self.a = result;
            }

            self.status.set(StatusFlags::NEGATIVE, result & 0x80 != 0);
            self.status.set(StatusFlags::ZERO, result == 0);
            self.status.set(StatusFlags::CARRY, $carry_check(value));
        }
    };
}

macro_rules! store {
    ($fn_name:ident, $reg:ident) => {
        pub fn $fn_name(&mut self, mode: AddressMode) {
            let (addr, _) = self.get_operand(mode);
            self.memory[addr as usize] = self.$reg;
        }
    };
}

macro_rules! transfer {
    ($fn_name:ident, $source:ident, $dest:ident) => {
        pub fn $fn_name(&mut self) {
            self.$dest = self.$source;
            self.update_zero_and_negative_flags(self.$dest);
        }
    };
}

macro_rules! increment {
    (inc, $reg:ident) => {
        pub fn inc(&mut self, mode: AddressMode) {
            let (addr, mut value) = self.get_operand(mode);
            value = value.wrapping_add(1);
            self.memory[addr as usize] = value;
            self.update_zero_and_negative_flags(value);
        }
    };
    ($fn_name:ident, $reg:ident) => {
        pub fn $fn_name(&mut self) {
            self.$reg = self.$reg.wrapping_add(1);
            self.update_zero_and_negative_flags(self.$reg);
        }
    };
}

macro_rules! decrement {
    (dec, $reg:ident) => {
        pub fn dec(&mut self, mode: AddressMode) {
            let (addr, mut value) = self.get_operand(mode);
            value = value.wrapping_sub(1);
            self.memory[addr as usize] = value;
            self.update_zero_and_negative_flags(value);
        }
    };
    ($fn_name:ident, $reg:ident) => {
        pub fn $fn_name(&mut self) {
            self.$reg = self.$reg.wrapping_sub(1);
            self.update_zero_and_negative_flags(self.$reg);
        }
    };
}

macro_rules! set_flag {
    ($fn_name:ident, $flag:ident, $value:expr) => {
        pub fn $fn_name(&mut self) {
            self.status.set(StatusFlags::$flag, $value);
        }
    };
}

impl CPU {
    increment!(inc, addr);
    increment!(inx, x);
    increment!(iny, y);

    decrement!(dec, addr);
    decrement!(dex, x);
    decrement!(dey, y);

    compare!(cmp, a);
    compare!(cpx, x);
    compare!(cpy, y);

    logical!(and, &);
    logical!(eor, ^);
    logical!(ora, |);

    rotate!(ror, rotate_right, |value| value & 0x01 != 0);
    rotate!(rol, rotate_left, |value| value & 0x80 != 0);

    load_register!(lda, a);
    load_register!(ldx, x);
    load_register!(ldy, y);

    store!(sta, a);
    store!(stx, x);
    store!(sty, y);

    transfer!(tax, a, x);
    transfer!(tay, a, y);
    transfer!(tsx, sp, x);
    transfer!(txa, x, a);
    transfer!(tya, y, a);

    branch!(bcc, CARRY, |c: bool| !c);
    branch!(bcs, CARRY, |c: bool| c);
    branch!(beq, ZERO, |z: bool| z);
    branch!(bmi, NEGATIVE, |n: bool| n);
    branch!(bne, ZERO, |z: bool| !z);
    branch!(bpl, NEGATIVE, |n: bool| !n);
    branch!(bvc, OVERFLOW, |v: bool| !v);
    branch!(bvs, OVERFLOW, |v| v);

    set_flag!(clc, CARRY, false);
    set_flag!(cld, DECIMAL, false);
    set_flag!(cli, INTERRUPT, false);
    set_flag!(clv, OVERFLOW, false);

    set_flag!(sec, CARRY, true);
    set_flag!(sed, DECIMAL, true);
    set_flag!(sei, INTERRUPT, true);

    pub fn adc(&mut self, mode: AddressMode) {
        let (_, value) = self.get_operand(mode);

        let carry = self.status.contains(StatusFlags::CARRY) as u16;
        let sum = self.a as u16 + value as u16 + carry;
        let result = sum as u8;

        self.a = result;

        self.update_zero_and_negative_flags(self.a);
        self.update_carry_flag(sum);
        self.update_overflow_flag(value, result);
    }

    pub fn asl(&mut self, mode: AddressMode) {
        let (addr, mut value) = self.get_operand(mode);
        self.status.set(StatusFlags::CARRY, (value & 0x80) != 0);
        value <<= 1;
        if addr != 0 {
            self.memory[addr as usize] = value;
        } else {
            self.a = value;
        }
        self.update_zero_and_negative_flags(value);
    }

    pub fn bit(&mut self, mode: AddressMode) {
        let (_, value) = self.get_operand(mode);
        self.status.set(StatusFlags::ZERO, (self.a & value) == 0);
        self.status.set(StatusFlags::NEGATIVE, (value & 0x80) != 0);
        self.status.set(StatusFlags::OVERFLOW, (value & 0x40) != 0);
    }

    pub fn brk(&mut self) {
        self.pc += 1;
        self.push_word(self.pc); // Push PC + 2 to the stack
        self.status.insert(StatusFlags::BREAK);
        self.status.insert(StatusFlags::INTERRUPT);
        self.push_status();
        self.pc = self.read_word(0xFFFE); // Read the interrupt vector address from 0xFFFE/0xFFFF
    }

    pub fn jmp(&mut self, mode: AddressMode) {
        let (addr, _) = self.get_operand(mode);
        self.pc = addr;
    }

    pub fn jsr(&mut self) {
        let addr = self.fetch_word();
        self.push_word(self.pc.wrapping_sub(1));
        self.pc = addr;
    }

    pub fn lsr(&mut self, mode: AddressMode) {
        let (addr, value) = self.get_operand(mode);

        let result = value >> 1;

        if addr != 0 {
            self.memory[addr as usize] = result;
        } else {
            self.a = result;
        }
        self.status.set(StatusFlags::CARRY, value & 0x01 != 0);
        self.status.set(StatusFlags::ZERO, result == 0);
        self.status.set(StatusFlags::NEGATIVE, false);
    }

    pub fn nop(&mut self) {}

    pub fn pha(&mut self) {
        self.push(self.a);
    }

    pub fn php(&mut self) {
        self.push_status()
    }

    pub fn pla(&mut self) {
        self.a = self.pop();
        self.status.set(StatusFlags::NEGATIVE, self.a & 0x80 != 0);
        self.status.set(StatusFlags::ZERO, self.a == 0);
    }

    pub fn plp(&mut self) {
        self.status = StatusFlags::from_bits_truncate(self.pop());
        // Clear the BREAK and UNUSED flags
        self.status.remove(StatusFlags::BREAK);
        self.status.remove(StatusFlags::UNUSED);
    }

    pub fn rti(&mut self) {
        self.status = StatusFlags::from_bits_truncate(self.pop());
        self.status.remove(StatusFlags::BREAK | StatusFlags::UNUSED);

        let pcl = self.pop() as u16;
        let pch = self.pop() as u16;
        self.pc = (pch << 8) | pcl;
    }

    pub fn rts(&mut self) {
        let pcl = self.pop() as u16;
        let pch = self.pop() as u16;
        self.pc = (pch << 8) | pcl;
        self.pc += 1;
    }

    pub fn sbc(&mut self, mode: AddressMode) {
        let (_, value) = self.get_operand(mode);

        let carry = !self.status.contains(StatusFlags::CARRY) as u8;
        let result = self.a.wrapping_sub(value).wrapping_sub(carry);

        self.status
            .set(StatusFlags::CARRY, self.a >= (value + carry));
        self.status.set(StatusFlags::ZERO, result == 0);
        self.status.set(StatusFlags::NEGATIVE, result & 0x80 != 0);

        let overflow = ((self.a ^ result) & 0x80 != 0) && ((self.a ^ value) & 0x80 != 0);
        self.status.set(StatusFlags::OVERFLOW, overflow);

        self.a = result;
    }

    pub fn txs(&mut self) {
        self.sp = self.x;
    }
}
