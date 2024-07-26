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

impl CPU {
    pub fn adc(&mut self, mode: AddressMode) {
        let value = match mode {
            AddressMode::Immediate => self.fetch_byte(),
            AddressMode::ZeroPage => {
                let addr = self.fetch_byte() as u16;
                self.read_byte(addr)
            }
            AddressMode::ZeroPageX => {
                let addr = self.fetch_byte().wrapping_add(self.x) as u16;
                self.read_byte(addr)
            }
            AddressMode::Absolute => {
                let addr = self.fetch_word();
                self.read_byte(addr)
            }
            AddressMode::AbsoluteX => {
                let addr = self.fetch_word().wrapping_add(self.x as u16);
                self.read_byte(addr)
            }
            AddressMode::AbsoluteY => {
                let addr = self.fetch_word().wrapping_add(self.y as u16);
                self.read_byte(addr)
            }
            AddressMode::IndirectX => {
                let base = self.fetch_byte().wrapping_add(self.x);
                let lo = self.read_byte(base as u16) as u16;
                let hi = self.read_byte(base.wrapping_add(1) as u16) as u16;
                let addr = (hi << 8) | lo;
                self.read_byte(addr)
            }
            AddressMode::IndirectY => {
                let base = self.fetch_byte();
                let lo = self.read_byte(base as u16) as u16;
                let hi = self.read_byte(base.wrapping_add(1) as u16) as u16;
                let addr = ((hi << 8) | lo).wrapping_add(self.y as u16);
                self.read_byte(addr)
            }
            _ => unimplemented!(),
        };

        let carry = if self.status.contains(StatusFlags::CARRY) {
            1
        } else {
            0
        };
        let sum = self.a as u16 + value as u16 + carry;
        let result = sum as u8;

        // Update accumulator
        self.a = result;

        // Update flags
        self.update_zero_and_negative_flags(self.a);
        self.update_carry_flag(sum);
        self.update_overflow_flag(value, result);
    }

    pub fn cmp(&mut self, mode: AddressMode) {
        let value = match mode {
            AddressMode::Immediate => self.fetch_byte(),
            AddressMode::ZeroPage => {
                let addr = self.fetch_byte() as u16;
                self.read_byte(addr)
            }
            AddressMode::ZeroPageX => {
                let addr = self.fetch_byte().wrapping_add(self.x) as u16;
                self.read_byte(addr)
            }
            AddressMode::Absolute => {
                let addr = self.fetch_word();
                self.read_byte(addr)
            }
            AddressMode::AbsoluteX => {
                let addr = self.fetch_word().wrapping_add(self.x as u16);
                self.read_byte(addr)
            }
            AddressMode::AbsoluteY => {
                let addr = self.fetch_word().wrapping_add(self.y as u16);
                self.read_byte(addr)
            }
            AddressMode::IndirectX => {
                let base = self.fetch_byte().wrapping_add(self.x);
                let lo = self.read_byte(base as u16) as u16;
                let hi = self.read_byte(base.wrapping_add(1) as u16) as u16;
                let addr = (hi << 8) | lo;
                self.read_byte(addr)
            }
            AddressMode::IndirectY => {
                let base = self.fetch_byte();
                let lo = self.read_byte(base as u16) as u16;
                let hi = self.read_byte(base.wrapping_add(1) as u16) as u16;
                let addr = ((hi << 8) | lo).wrapping_add(self.y as u16);
                self.read_byte(addr)
            }
            _ => unimplemented!(),
        };

        let result = self.a.wrapping_sub(value);

        self.status.set(StatusFlags::CARRY, self.a >= value);
        self.status.set(StatusFlags::ZERO, self.a == value);
        self.status.set(StatusFlags::NEGATIVE, result & 0x80 != 0);
    }

    pub fn cpx(&mut self, mode: AddressMode) {
        let value = match mode {
            AddressMode::Immediate => self.fetch_byte(),
            AddressMode::ZeroPage => {
                let addr = self.fetch_byte() as u16;
                self.read_byte(addr)
            }
            AddressMode::Absolute => {
                let addr = self.fetch_word();
                self.read_byte(addr)
            }
            _ => unimplemented!(),
        };

        let result = self.x.wrapping_sub(value);

        self.status.set(StatusFlags::CARRY, self.x >= value);
        self.status.set(StatusFlags::ZERO, self.x == value);
        self.status.set(StatusFlags::NEGATIVE, result & 0x80 != 0);
    }

    pub fn cpy(&mut self, mode: AddressMode) {
        let value = match mode {
            AddressMode::Immediate => self.fetch_byte(),
            AddressMode::ZeroPage => {
                let addr = self.fetch_byte() as u16;
                self.read_byte(addr)
            }
            AddressMode::Absolute => {
                let addr = self.fetch_word();
                self.read_byte(addr)
            }
            _ => unimplemented!(),
        };

        let result = self.y.wrapping_sub(value);

        self.status.set(StatusFlags::CARRY, self.y >= value);
        self.status.set(StatusFlags::ZERO, self.y == value);
        self.status.set(StatusFlags::NEGATIVE, result & 0x80 != 0);
    }

    pub fn lda(&mut self, mode: AddressMode) {
        let value = match mode {
            AddressMode::Immediate => self.fetch_byte(),
            AddressMode::ZeroPage => {
                let addr = self.fetch_byte() as u16;
                self.read_byte(addr)
            }
            AddressMode::ZeroPageX => {
                let addr = self.fetch_byte().wrapping_add(self.x) as u16;
                self.read_byte(addr)
            }
            AddressMode::Absolute => {
                let addr = self.fetch_word();
                self.read_byte(addr)
            }
            AddressMode::AbsoluteX => {
                let addr = self.fetch_word().wrapping_add(self.x as u16);
                self.read_byte(addr)
            }
            AddressMode::AbsoluteY => {
                let addr = self.fetch_word().wrapping_add(self.y as u16);
                self.read_byte(addr)
            }
            AddressMode::IndirectX => {
                let base = self.fetch_byte().wrapping_add(self.x);
                let lo = self.read_byte(base as u16) as u16;
                let hi = self.read_byte(base.wrapping_add(1) as u16) as u16;
                let addr = (hi << 8) | lo;
                self.read_byte(addr)
            }
            AddressMode::IndirectY => {
                let base = self.fetch_byte();
                let lo = self.read_byte(base as u16) as u16;
                let hi = self.read_byte(base.wrapping_add(1) as u16) as u16;
                let addr = ((hi << 8) | lo).wrapping_add(self.y as u16);
                self.read_byte(addr)
            }
            _ => unimplemented!(),
        };
        self.a = value;
        self.update_zero_and_negative_flags(self.a);
    }

    pub fn ldx(&mut self, mode: AddressMode) {
        let value = match mode {
            AddressMode::Immediate => self.fetch_byte(),
            AddressMode::ZeroPage => {
                let addr = self.fetch_byte() as u16;
                self.read_byte(addr)
            }
            AddressMode::ZeroPageY => {
                let addr = self.fetch_byte().wrapping_add(self.y) as u16;
                self.read_byte(addr)
            }
            AddressMode::Absolute => {
                let addr = self.fetch_word();
                self.read_byte(addr)
            }
            AddressMode::AbsoluteY => {
                let addr = self.fetch_word().wrapping_add(self.y as u16);
                self.read_byte(addr)
            }
            _ => unimplemented!(),
        };
        self.x = value;
        self.update_zero_and_negative_flags(self.x);
    }

    pub fn ldy(&mut self, mode: AddressMode) {
        let value = match mode {
            AddressMode::Immediate => self.fetch_byte(),
            AddressMode::ZeroPage => {
                let addr = self.fetch_byte() as u16;
                self.read_byte(addr)
            }
            AddressMode::ZeroPageX => {
                let addr = self.fetch_byte().wrapping_add(self.x) as u16;
                self.read_byte(addr)
            }
            AddressMode::Absolute => {
                let addr = self.fetch_word();
                self.read_byte(addr)
            }
            AddressMode::AbsoluteX => {
                let addr = self.fetch_word().wrapping_add(self.x as u16);
                self.read_byte(addr)
            }
            _ => unimplemented!(),
        };
        self.y = value;
        self.update_zero_and_negative_flags(self.y);
    }

    pub fn and(&mut self, mode: AddressMode) {
        let value = match mode {
            AddressMode::Immediate => self.fetch_byte(),
            AddressMode::ZeroPage => {
                let addr = self.fetch_byte() as u16;
                self.read_byte(addr)
            }
            AddressMode::ZeroPageX => {
                let addr = self.fetch_byte().wrapping_add(self.x) as u16;
                self.read_byte(addr)
            }
            AddressMode::Absolute => {
                let addr = self.fetch_word();
                self.read_byte(addr)
            }
            AddressMode::AbsoluteX => {
                let addr = self.fetch_word().wrapping_add(self.x as u16);
                self.read_byte(addr)
            }
            AddressMode::AbsoluteY => {
                let addr = self.fetch_word().wrapping_add(self.y as u16);
                self.read_byte(addr)
            }
            AddressMode::IndirectX => {
                let base = self.fetch_byte().wrapping_add(self.x);
                let lo = self.read_byte(base as u16) as u16;
                let hi = self.read_byte(base.wrapping_add(1) as u16) as u16;
                let addr = (hi << 8) | lo;
                self.read_byte(addr)
            }
            AddressMode::IndirectY => {
                let base = self.fetch_byte();
                let lo = self.read_byte(base as u16) as u16;
                let hi = self.read_byte(base.wrapping_add(1) as u16) as u16;
                let addr = ((hi << 8) | lo).wrapping_add(self.y as u16);
                self.read_byte(addr)
            }
            _ => unimplemented!(),
        };

        self.a &= value;
        self.update_zero_and_negative_flags(self.a);
    }

    pub fn ora(&mut self, mode: AddressMode) {
        let value = match mode {
            AddressMode::Immediate => self.fetch_byte(),
            AddressMode::ZeroPage => {
                let addr = self.fetch_byte() as u16;
                self.read_byte(addr)
            }
            AddressMode::ZeroPageX => {
                let addr = self.fetch_byte().wrapping_add(self.x) as u16;
                self.read_byte(addr)
            }
            AddressMode::Absolute => {
                let addr = self.fetch_word();
                self.read_byte(addr)
            }
            AddressMode::AbsoluteX => {
                let addr = self.fetch_word().wrapping_add(self.x as u16);
                self.read_byte(addr)
            }
            AddressMode::AbsoluteY => {
                let addr = self.fetch_word().wrapping_add(self.y as u16);
                self.read_byte(addr)
            }
            AddressMode::IndirectX => {
                let base = self.fetch_byte().wrapping_add(self.x);
                let lo = self.read_byte(base as u16) as u16;
                let hi = self.read_byte(base.wrapping_add(1) as u16) as u16;
                let addr = (hi << 8) | lo;
                self.read_byte(addr)
            }
            AddressMode::IndirectY => {
                let base = self.fetch_byte();
                let lo = self.read_byte(base as u16) as u16;
                let hi = self.read_byte(base.wrapping_add(1) as u16) as u16;
                let addr = ((hi << 8) | lo).wrapping_add(self.y as u16);
                self.read_byte(addr)
            }
            _ => unimplemented!(),
        };

        self.a |= value;
        self.status.set(StatusFlags::ZERO, self.a == 0);
        self.status.set(StatusFlags::NEGATIVE, self.a & 0x80 != 0);
    }

    pub fn eor(&mut self, mode: AddressMode) {
        let value = match mode {
            AddressMode::Immediate => self.fetch_byte(),
            AddressMode::ZeroPage => {
                let addr = self.fetch_byte() as u16;
                self.read_byte(addr)
            }
            AddressMode::ZeroPageX => {
                let addr = self.fetch_byte().wrapping_add(self.x) as u16;
                self.read_byte(addr)
            }
            AddressMode::Absolute => {
                let addr = self.fetch_word();
                self.read_byte(addr)
            }
            AddressMode::AbsoluteX => {
                let addr = self.fetch_word().wrapping_add(self.x as u16);
                self.read_byte(addr)
            }
            AddressMode::AbsoluteY => {
                let addr = self.fetch_word().wrapping_add(self.y as u16);
                self.read_byte(addr)
            }
            AddressMode::IndirectX => {
                let base = self.fetch_byte().wrapping_add(self.x);
                let lo = self.read_byte(base as u16) as u16;
                let hi = self.read_byte(base.wrapping_add(1) as u16) as u16;
                let addr = (hi << 8) | lo;
                self.read_byte(addr)
            }
            AddressMode::IndirectY => {
                let base = self.fetch_byte();
                let lo = self.read_byte(base as u16) as u16;
                let hi = self.read_byte(base.wrapping_add(1) as u16) as u16;
                let addr = ((hi << 8) | lo).wrapping_add(self.y as u16);
                self.read_byte(addr)
            }
            _ => unimplemented!(),
        };

        self.a ^= value;
        self.update_zero_and_negative_flags(self.a);
    }

    pub fn asl(&mut self, mode: AddressMode) {
        match mode {
            AddressMode::Accumulator => {
                let mut value = self.a;
                self.status.set(StatusFlags::CARRY, (value & 0x80) != 0);
                value <<= 1;
                self.a = value;
                self.update_zero_and_negative_flags(value);
            }
            AddressMode::ZeroPage => {
                let addr = self.fetch_byte() as u16;
                let mut value = self.read_byte(addr);
                self.status.set(StatusFlags::CARRY, (value & 0x80) != 0);
                value <<= 1;
                self.memory[addr as usize] = value;
                self.update_zero_and_negative_flags(value);
            }
            AddressMode::ZeroPageX => {
                let addr = self.fetch_byte().wrapping_add(self.x) as u16;
                let mut value = self.read_byte(addr);
                self.status.set(StatusFlags::CARRY, (value & 0x80) != 0);
                value <<= 1;
                self.memory[addr as usize] = value;
                self.update_zero_and_negative_flags(value);
            }
            AddressMode::Absolute => {
                let addr = self.fetch_word();
                let mut value = self.read_byte(addr);
                self.status.set(StatusFlags::CARRY, (value & 0x80) != 0);
                value <<= 1;
                self.memory[addr as usize] = value;
                self.update_zero_and_negative_flags(value);
            }
            AddressMode::AbsoluteX => {
                let addr = self.fetch_word().wrapping_add(self.x as u16);
                let mut value = self.read_byte(addr);
                self.status.set(StatusFlags::CARRY, (value & 0x80) != 0);
                value <<= 1;
                self.memory[addr as usize] = value;
                self.update_zero_and_negative_flags(value);
            }
            _ => panic!("Addressing mode not supported for ASL instruction"),
        };
    }

    pub fn bcc(&mut self, mode: AddressMode) {
        match mode {
            AddressMode::Relative => {
                let offset = self.fetch_byte() as i8;
                if !self.status.contains(StatusFlags::CARRY) {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            _ => panic!("Addressing mode not supported for BCC instruction"),
        }
    }

    pub fn bcs(&mut self, mode: AddressMode) {
        match mode {
            AddressMode::Relative => {
                let offset = self.fetch_byte() as i8;
                if self.status.contains(StatusFlags::CARRY) {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            _ => panic!("Addressing mode not supported for BCS instruction"),
        }
    }

    pub fn beq(&mut self, mode: AddressMode) {
        match mode {
            AddressMode::Relative => {
                let offset = self.fetch_byte() as i8;
                if self.status.contains(StatusFlags::ZERO) {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            _ => panic!("Addressing mode not supported for BEQ instruction"),
        }
    }

    pub fn bmi(&mut self, mode: AddressMode) {
        match mode {
            AddressMode::Relative => {
                let offset = self.fetch_byte() as i8;
                if self.status.contains(StatusFlags::NEGATIVE) {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            _ => panic!("Addressing mode not supported for BMI instruction"),
        }
    }

    pub fn bne(&mut self, mode: AddressMode) {
        match mode {
            AddressMode::Relative => {
                let offset = self.fetch_byte() as i8;
                if !self.status.contains(StatusFlags::ZERO) {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            _ => panic!("Addressing mode not supported for BNE instruction"),
        }
    }

    pub fn bpl(&mut self, mode: AddressMode) {
        match mode {
            AddressMode::Relative => {
                let offset = self.fetch_byte() as i8;
                if !self.status.contains(StatusFlags::NEGATIVE) {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            _ => panic!("Addressing mode not supported for BNE instruction"),
        }
    }

    pub fn bvc(&mut self, mode: AddressMode) {
        match mode {
            AddressMode::Relative => {
                let offset = self.fetch_byte() as i8;
                if !self.status.contains(StatusFlags::OVERFLOW) {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            _ => panic!("Addressing mode not supported for BNE instruction"),
        }
    }

    pub fn bvs(&mut self, mode: AddressMode) {
        match mode {
            AddressMode::Relative => {
                let offset = self.fetch_byte() as i8;
                if self.status.contains(StatusFlags::OVERFLOW) {
                    self.pc = self.pc.wrapping_add(offset as u16);
                }
            }
            _ => panic!("Addressing mode not supported for BNE instruction"),
        }
    }

    pub fn bit(&mut self, mode: AddressMode) {
        match mode {
            AddressMode::ZeroPage => {
                let addr = self.fetch_byte() as u16;
                let value = self.read_byte(addr);

                self.status.set(StatusFlags::ZERO, (self.a & value) == 0);
                self.status.set(StatusFlags::NEGATIVE, (value & 0x80) != 0);
                self.status.set(StatusFlags::OVERFLOW, (value & 0x40) != 0);
            }
            AddressMode::Absolute => {
                let addr = self.fetch_word();
                let value = self.read_byte(addr);

                self.status.set(StatusFlags::ZERO, (self.a & value) == 0);
                self.status.set(StatusFlags::NEGATIVE, (value & 0x80) != 0);
                self.status.set(StatusFlags::OVERFLOW, (value & 0x40) != 0);
            }
            _ => panic!("Addressing mode not supported for BCS instruction"),
        }
    }

    pub fn dec(&mut self, mode: AddressMode) {
        let (value, addr) = match mode {
            AddressMode::ZeroPage => {
                let addr = self.fetch_byte() as u16;
                (self.read_byte(addr), addr)
            }
            AddressMode::ZeroPageX => {
                let addr = self.fetch_byte().wrapping_add(self.x) as u16;
                (self.read_byte(addr), addr)
            }
            AddressMode::Absolute => {
                let addr = self.fetch_word();
                (self.read_byte(addr), addr)
            }
            AddressMode::AbsoluteX => {
                let addr = self.fetch_word().wrapping_add(self.x as u16);
                (self.read_byte(addr), addr)
            }
            _ => unimplemented!(),
        };

        let result = value.wrapping_sub(1);

        self.memory[addr as usize] = result;

        self.status.set(StatusFlags::ZERO, result == 0);
        self.status.set(StatusFlags::NEGATIVE, result & 0x80 != 0);
    }

    pub fn dex(&mut self) {
        let result = self.x.wrapping_sub(1);

        self.x = result;

        self.status.set(StatusFlags::ZERO, result == 0);
        self.status.set(StatusFlags::NEGATIVE, result & 0x80 != 0);
    }

    pub fn dey(&mut self) {
        let result = self.y.wrapping_sub(1);

        self.y = result;

        self.status.set(StatusFlags::ZERO, result == 0);
        self.status.set(StatusFlags::NEGATIVE, result & 0x80 != 0);
    }

    pub fn inc(&mut self, mode: AddressMode) {
        let (value, addr) = match mode {
            AddressMode::ZeroPage => {
                let addr = self.fetch_byte() as u16;
                (self.read_byte(addr), addr)
            }
            AddressMode::ZeroPageX => {
                let addr = self.fetch_byte().wrapping_add(self.x) as u16;
                (self.read_byte(addr), addr)
            }
            AddressMode::Absolute => {
                let addr = self.fetch_word();
                (self.read_byte(addr), addr)
            }
            AddressMode::AbsoluteX => {
                let addr = self.fetch_word().wrapping_add(self.x as u16);
                (self.read_byte(addr), addr)
            }
            _ => unimplemented!(),
        };

        let result = value.wrapping_add(1);

        self.memory[addr as usize] = result;

        self.status.set(StatusFlags::ZERO, result == 0);
        self.status.set(StatusFlags::NEGATIVE, result & 0x80 != 0);
    }

    pub fn inx(&mut self) {
        let result = self.x.wrapping_add(1);

        self.x = result;

        self.status.set(StatusFlags::ZERO, result == 0);
        self.status.set(StatusFlags::NEGATIVE, result & 0x80 != 0);
    }

    pub fn iny(&mut self) {
        let result = self.y.wrapping_add(1);

        self.y = result;

        self.status.set(StatusFlags::ZERO, result == 0);
        self.status.set(StatusFlags::NEGATIVE, result & 0x80 != 0);
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
        match mode {
            AddressMode::Absolute => {
                let addr = self.fetch_word();
                self.pc = addr;
            }
            AddressMode::Indirect => {
                let base = self.fetch_word();
                let lo = self.read_byte(base) as u16;
                let hi = self.read_byte(base.wrapping_add(1)) as u16;
                let addr = (hi << 8) | lo;
                self.pc = addr;
            }
            _ => unimplemented!(),
        }
    }

    pub fn jsr(&mut self) {
        let addr = self.fetch_word();
        self.push_word(self.pc.wrapping_sub(1));
        self.pc = addr;
    }

    pub fn lsr(&mut self, mode: AddressMode) {
        let (value, result) = match mode {
            AddressMode::Accumulator => {
                let value = self.a;
                self.a >>= 1;
                (value, self.a)
            }
            AddressMode::ZeroPage => {
                let addr = self.fetch_byte() as u16;
                let value = self.read_byte(addr);
                let result = value >> 1;
                self.memory[addr as usize] = result;
                (value, result)
            }
            AddressMode::ZeroPageX => {
                let addr = self.fetch_byte().wrapping_add(self.x) as u16;
                let value = self.read_byte(addr);
                let result = value >> 1;
                self.memory[addr as usize] = result;
                (value, result)
            }
            AddressMode::Absolute => {
                let addr = self.fetch_word();
                let value = self.read_byte(addr);
                let result = value >> 1;
                self.memory[addr as usize] = result;
                (value, result)
            }
            AddressMode::AbsoluteX => {
                let addr = self.fetch_word().wrapping_add(self.x as u16) as u16;
                let value = self.read_byte(addr);
                let result = value >> 1;
                self.memory[addr as usize] = result;
                (value, result)
            }
            _ => unimplemented!(),
        };

        self.status.set(StatusFlags::CARRY, value & 0x01 != 0);
        self.status.set(StatusFlags::ZERO, result == 0);
        self.status.set(StatusFlags::NEGATIVE, false);
    }

    pub fn clc(&mut self) {
        self.status.set(StatusFlags::CARRY, false);
    }

    pub fn cld(&mut self) {
        self.status.set(StatusFlags::DECIMAL, false);
    }

    pub fn cli(&mut self) {
        self.status.set(StatusFlags::INTERRUPT, false);
    }

    pub fn clv(&mut self) {
        self.status.set(StatusFlags::OVERFLOW, false);
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

    pub fn ror(&mut self, mode: AddressMode) {
        let (value, result) = match mode {
            AddressMode::Accumulator => {
                let value = self.a;
                self.a = self.rotate_right(value);
                (value, self.a)
            }
            AddressMode::ZeroPage => {
                let addr = self.fetch_byte() as u16;
                let value = self.read_byte(addr);
                let result = self.rotate_right(value);
                self.memory[addr as usize] = result;
                (value, result)
            }
            AddressMode::ZeroPageX => {
                let addr = self.fetch_byte().wrapping_add(self.x) as u16;
                let value = self.read_byte(addr);
                let result = self.rotate_right(value);
                self.memory[addr as usize] = result;
                (value, result)
            }
            AddressMode::Absolute => {
                let addr = self.fetch_word();
                let value = self.read_byte(addr);
                let result = self.rotate_right(value);
                self.memory[addr as usize] = result;
                (value, result)
            }
            AddressMode::AbsoluteX => {
                let addr = self.fetch_word().wrapping_add(self.x as u16);
                let value = self.read_byte(addr);
                let result = self.rotate_right(value);
                self.memory[addr as usize] = result;
                (value, result)
            }
            _ => unimplemented!(),
        };

        self.status.set(StatusFlags::NEGATIVE, result & 0x80 != 0);
        self.status.set(StatusFlags::ZERO, result == 0);
        self.status.set(StatusFlags::CARRY, value & 0x01 != 0);
    }

    pub fn rol(&mut self, mode: AddressMode) {
        let (value, result) = match mode {
            AddressMode::Accumulator => {
                let value = self.a;
                self.a = self.rotate_left(value);
                (value, self.a)
            }
            AddressMode::ZeroPage => {
                let addr = self.fetch_byte() as u16;
                let value = self.read_byte(addr);
                let result = self.rotate_left(value);
                self.memory[addr as usize] = result;
                (value, result)
            }
            AddressMode::ZeroPageX => {
                let addr = self.fetch_byte().wrapping_add(self.x) as u16;
                let value = self.read_byte(addr);
                let result = self.rotate_left(value);
                self.memory[addr as usize] = result;
                (value, result)
            }
            AddressMode::Absolute => {
                let addr = self.fetch_word();
                let value = self.read_byte(addr);
                let result = self.rotate_left(value);
                self.memory[addr as usize] = result;
                (value, result)
            }
            AddressMode::AbsoluteX => {
                let addr = self.fetch_word().wrapping_add(self.x as u16);
                let value = self.read_byte(addr);
                let result = self.rotate_left(value);
                self.memory[addr as usize] = result;
                (value, result)
            }
            _ => unimplemented!(),
        };

        self.status.set(StatusFlags::NEGATIVE, result & 0x80 != 0);
        self.status.set(StatusFlags::ZERO, result == 0);
        self.status.set(StatusFlags::CARRY, value & 0x80 != 0);
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
        let value = match mode {
            AddressMode::Immediate => self.fetch_byte(),
            AddressMode::ZeroPage => {
                let addr = self.fetch_byte() as u16;
                self.read_byte(addr)
            }
            AddressMode::ZeroPageX => {
                let addr = self.fetch_byte().wrapping_add(self.x) as u16;
                self.read_byte(addr)
            }
            AddressMode::Absolute => {
                let addr = self.fetch_word();
                self.read_byte(addr)
            }
            AddressMode::AbsoluteX => {
                let addr = self.fetch_word().wrapping_add(self.x as u16);
                self.read_byte(addr)
            }
            AddressMode::AbsoluteY => {
                let addr = self.fetch_word().wrapping_add(self.y as u16);
                self.read_byte(addr)
            }
            AddressMode::IndirectX => {
                let base = self.fetch_byte().wrapping_add(self.x);
                let lo = self.read_byte(base as u16) as u16;
                let hi = self.read_byte(base.wrapping_add(1) as u16) as u16;
                let addr = (hi << 8) | lo;
                self.read_byte(addr)
            }
            AddressMode::IndirectY => {
                let base = self.fetch_byte();
                let lo = self.read_byte(base as u16) as u16;
                let hi = self.read_byte(base.wrapping_add(1) as u16) as u16;
                let addr = ((hi << 8) | lo).wrapping_add(self.y as u16);
                self.read_byte(addr)
            }
            _ => unimplemented!(),
        };

        let carry = if self.status.contains(StatusFlags::CARRY) {
            0
        } else {
            1
        };
        let result = self.a.wrapping_sub(value).wrapping_sub(carry);

        self.status
            .set(StatusFlags::CARRY, self.a >= (value + carry));
        self.status.set(StatusFlags::ZERO, result == 0);
        self.status.set(StatusFlags::NEGATIVE, result & 0x80 != 0);

        let overflow = ((self.a ^ result) & 0x80 != 0) && ((self.a ^ value) & 0x80 != 0);
        self.status.set(StatusFlags::OVERFLOW, overflow);

        self.a = result;
    }

    pub fn sta(&mut self, mode: AddressMode) {
        match mode {
            AddressMode::ZeroPage => {
                let addr = self.fetch_byte() as u16;
                self.memory[addr as usize] = self.a;
            }
            AddressMode::ZeroPageX => {
                let addr = self.fetch_byte().wrapping_add(self.x) as u16;
                self.memory[addr as usize] = self.a;
            }
            AddressMode::Absolute => {
                let addr = self.fetch_word();
                self.memory[addr as usize] = self.a;
            }
            AddressMode::AbsoluteX => {
                let addr = self.fetch_word().wrapping_add(self.x as u16);
                self.memory[addr as usize] = self.a;
            }
            AddressMode::AbsoluteY => {
                let addr = self.fetch_word().wrapping_add(self.y as u16);
                self.memory[addr as usize] = self.a;
            }
            AddressMode::IndirectX => {
                let base = self.fetch_byte().wrapping_add(self.x);
                let lo = self.read_byte(base as u16) as u16;
                let hi = self.read_byte(base.wrapping_add(1) as u16) as u16;
                let addr = (hi << 8) | lo;
                self.memory[addr as usize] = self.a;
            }
            AddressMode::IndirectY => {
                let base = self.fetch_byte();
                let lo = self.read_byte(base as u16) as u16;
                let hi = self.read_byte(base.wrapping_add(1) as u16) as u16;
                let addr = ((hi << 8) | lo).wrapping_add(self.y as u16);
                self.memory[addr as usize] = self.a;
            }
            _ => unimplemented!(),
        }
    }

    pub fn stx(&mut self, mode: AddressMode) {
        match mode {
            AddressMode::ZeroPage => {
                let addr = self.fetch_byte() as u16;
                self.memory[addr as usize] = self.x;
            }
            AddressMode::ZeroPageY => {
                let addr = self.fetch_byte().wrapping_add(self.y) as u16;
                self.memory[addr as usize] = self.x;
            }
            AddressMode::Absolute => {
                let addr = self.fetch_word();
                self.memory[addr as usize] = self.x;
            }
            _ => unimplemented!(),
        }
    }

    pub fn sty(&mut self, mode: AddressMode) {
        match mode {
            AddressMode::ZeroPage => {
                let addr = self.fetch_byte() as u16;
                self.memory[addr as usize] = self.y;
            }
            AddressMode::ZeroPageX => {
                let addr = self.fetch_byte().wrapping_add(self.x) as u16;
                self.memory[addr as usize] = self.y;
            }
            AddressMode::Absolute => {
                let addr = self.fetch_word();
                self.memory[addr as usize] = self.y;
            }
            _ => unimplemented!(),
        }
    }

    pub fn tax(&mut self) {
        self.x = self.a;
        self.update_zero_and_negative_flags(self.x);
    }

    pub fn tay(&mut self) {
        self.y = self.a;
        self.update_zero_and_negative_flags(self.y);
    }

    pub fn tsx(&mut self) {
        self.x = self.sp;
        self.update_zero_and_negative_flags(self.x);
    }

    pub fn txa(&mut self) {
        self.a = self.x;
        self.update_zero_and_negative_flags(self.a);
    }

    pub fn txs(&mut self) {
        self.sp = self.x;
    }

    pub fn tya(&mut self) {
        self.a = self.y;
        self.update_zero_and_negative_flags(self.a);
    }

    pub fn sec(&mut self) {
        self.status.set(StatusFlags::CARRY, true);
    }

    pub fn sed(&mut self) {
        self.status.set(StatusFlags::DECIMAL, true);
    }

    pub fn sei(&mut self) {
        self.status.set(StatusFlags::INTERRUPT, true);
    }
}
