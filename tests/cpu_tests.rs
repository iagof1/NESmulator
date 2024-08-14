#[cfg(test)]
mod tests {
    use nes::cpu::instructions::AddressMode;
    use nes::cpu::{StatusFlags, CPU};
    use nes::rom::NesFile;
    use std::error::Error;

    #[test]
    fn test_adc_immediate_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x10;
        nes_cpu.status.insert(StatusFlags::CARRY);

        let opcode: u8 = 0x69;
        let operand: u8 = 0x20;

        nes_cpu.memory_bus.mem_write(nes_cpu.pc, opcode);
        nes_cpu
            .memory_bus
            .mem_write(nes_cpu.pc + 1, operand);
        nes_cpu.run();

        assert_eq!(nes_cpu.a, 0x10 + 0x20 + 1);
        assert!(nes_cpu.status.contains(StatusFlags::CARRY) == false);
        assert!(nes_cpu.status.contains(StatusFlags::OVERFLOW) == false);

        Ok(())
    }

    #[test]
    fn test_lda_immediate_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        let opcode: u8 = 0xA9;
        let operand: u8 = 0x11;

        nes_cpu.memory_bus.mem_write(nes_cpu.pc, opcode);
        nes_cpu
            .memory_bus
            .mem_write(nes_cpu.pc + 1, operand);
        nes_cpu.run();
        nes_cpu.run();
        assert_eq!(nes_cpu.a, 0x11);

        Ok(())
    }

    #[test]
    fn test_ldx_immediate_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        let opcode: u8 = 0xA2;
        let operand: u8 = 0x11;

        nes_cpu.memory_bus.mem_write(nes_cpu.pc, opcode);
        nes_cpu
            .memory_bus
            .mem_write(nes_cpu.pc + 1, operand);
        nes_cpu.run();
        nes_cpu.run();
        assert_eq!(nes_cpu.x, 0x11);

        Ok(())
    }

    #[test]
    fn test_ldy_immediate_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        let opcode: u8 = 0xA0;
        let operand: u8 = 0x11;

        nes_cpu.memory_bus.mem_write(nes_cpu.pc, opcode);
        nes_cpu
            .memory_bus
            .mem_write(nes_cpu.pc + 1, operand);
        nes_cpu.run();
        nes_cpu.run();
        assert_eq!(nes_cpu.y, 0x11);

        Ok(())
    }

    #[test]
    fn test_and_immediate_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x10;
        let opcode: u8 = 0x29;
        let operand: u8 = 0x10;

        nes_cpu.memory_bus.mem_write(nes_cpu.pc, opcode);
        nes_cpu
            .memory_bus
            .mem_write(nes_cpu.pc + 1, operand);
        nes_cpu.run();
        nes_cpu.run();

        assert_eq!(nes_cpu.a, 0x10);

        Ok(())
    }

    #[test]
    fn test_eor_immediate_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x55;
        nes_cpu.memory_bus.mem_write(0x0000, 0x49); // EOR Immediate opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x0F); // Immediate value
        nes_cpu.run();
        assert_eq!(nes_cpu.a, 0x5A);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));
        Ok(())
    }

    #[test]
    fn test_eor_zero_page_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x55;
        nes_cpu.pc = 0x0000; // Set program counter to a random start location
        nes_cpu.memory_bus.mem_write(0x0000, 0x45); // EOR ZeroPage opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x10); // ZeroPage address
        nes_cpu.memory_bus.mem_write(0x0010, 0x0F); // Value at address 0x0010
        nes_cpu.run();
        assert_eq!(nes_cpu.a, 0x5A);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));

        Ok(())
    }

    #[test]
    fn test_eor_zero_page_x_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );

        nes_cpu.a = 0x55;
        nes_cpu.x = 0x01;
        nes_cpu.memory_bus.mem_write(0x0000, 0x55); // EOR ZeroPageX opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x10); // ZeroPage base address
        nes_cpu.memory_bus.mem_write(0x0011, 0x0F); // Value at address 0x0010 + X
        nes_cpu.run();
        assert_eq!(nes_cpu.a, 0x5A);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));

        Ok(())
    }

    #[test]
    fn test_eor_absolute_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x55;
        nes_cpu.memory_bus.mem_write(0x0000, 0x4D); // EOR Absolute opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x00); // Low byte of address
        nes_cpu.memory_bus.mem_write(0x0002, 0x20); // High byte of address
        nes_cpu.memory_bus.mem_write(0x2000, 0x0F); // Value at address 0x2000
        nes_cpu.run();
        assert_eq!(nes_cpu.a, 0x5A);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));

        Ok(())
    }

    #[test]
    fn test_eor_absolute_x_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x55;
        nes_cpu.x = 0x01;
        nes_cpu.pc = 0x0000; // Set program counter to a random start location
        nes_cpu.memory_bus.mem_write(nes_cpu.pc, 0x5D); // EOR AbsoluteX opcode
        nes_cpu
            .memory_bus
            .mem_write(nes_cpu.pc + 1, 0x00); // Low byte of address
        nes_cpu
            .memory_bus
            .mem_write(nes_cpu.pc + 2, 0x01); // High byte of address
        nes_cpu.memory_bus.mem_write(0x0101, 0x0F); // Value at address 0x2000 + X
        nes_cpu.run();
        assert_eq!(nes_cpu.a, 0x5A);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));

        Ok(())
    }

    #[test]
    fn test_eor_absolute_y_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x55;
        nes_cpu.y = 0x01;
        nes_cpu.pc = 0x0000; // Set program counter to a random start location
        nes_cpu.memory_bus.mem_write(nes_cpu.pc, 0x59); // EOR AbsoluteY opcode
        nes_cpu
            .memory_bus
            .mem_write(nes_cpu.pc + 1, 0x00); // Low byte of address
        nes_cpu
            .memory_bus
            .mem_write(nes_cpu.pc + 2, 0x01); // High byte of address
        nes_cpu.memory_bus.mem_write(0x0101, 0x0F); // Value at address 0x2000 + Y
        nes_cpu.run();
        assert_eq!(nes_cpu.a, 0x5A);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));
        Ok(())
    }

    #[test]
    fn test_eor_indirect_x_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x55;
        nes_cpu.x = 0x04;
        nes_cpu.pc = 0x0000; // Set program counter to a random start location
        nes_cpu.memory_bus.mem_write(0x0000, 0x41); // EOR IndirectX opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x02); // Base address
        nes_cpu.memory_bus.mem_write(0x0006, 0x00); // Low byte of effective address
        nes_cpu.memory_bus.mem_write(0x0007, 0x20); // High byte of effective address
        nes_cpu.memory_bus.mem_write(0x2000, 0x0F); // Value at effective address
        nes_cpu.run();
        assert_eq!(nes_cpu.a, 0x5A);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));
        Ok(())
    }

    #[test]
    fn test_eor_indirect_y_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x55;
        nes_cpu.y = 0x04;
        nes_cpu.pc = 0x0000; // Set program counter to start location
        nes_cpu.memory_bus.mem_write(nes_cpu.pc, 0x51); // EOR IndirectY opcode
        nes_cpu
            .memory_bus
            .mem_write(nes_cpu.pc + 1, 0x02); // Zero page address

        // Set up the indirect address in zero page
        nes_cpu.memory_bus.mem_write(0x02, 0x00); // Low byte of indirect address
        nes_cpu.memory_bus.mem_write(0x03, 0x01); // High byte of indirect address

        // Write the value at the effective address
        nes_cpu.memory_bus.mem_write(0x0104, 0x0F); // Value at address (0x2000 + Y)

        nes_cpu.run();

        assert_eq!(nes_cpu.a, 0x5A); // Expected result of EOR (0x55 ^ 0x0F = 0x5A) assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));

        Ok(())
    }

    #[test]
    fn test_asl_accumulator_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0b0100_0001;
        nes_cpu.status.remove(StatusFlags::CARRY);

        nes_cpu.asl(AddressMode::Accumulator);

        assert_eq!(nes_cpu.a, 0b1000_0010);
        assert!(!nes_cpu.status.contains(StatusFlags::CARRY));
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(nes_cpu.status.contains(StatusFlags::NEGATIVE));
        Ok(())
    }

    #[test]
    fn test_asl_zero_page_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.memory_bus.mem_write(0x10, 0b0100_0001);
        nes_cpu.memory_bus.mem_write(0x00, 0x06);
        nes_cpu.memory_bus.mem_write(0x01, 0x10);

        nes_cpu.run();

        assert_eq!(nes_cpu.memory_bus.mem_read(0x10), 0b1000_0010);
        assert!(!nes_cpu.status.contains(StatusFlags::CARRY));
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(nes_cpu.status.contains(StatusFlags::NEGATIVE));
        Ok(())
    }

    #[test]
    fn test_bcc_relative_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.pc = 0x1000;

        nes_cpu.status.remove(StatusFlags::CARRY);

        nes_cpu.memory_bus.mem_write(nes_cpu.pc, 0x90);
        nes_cpu
            .memory_bus
            .mem_write(nes_cpu.pc + 1, 0x05);

        nes_cpu.run();

        assert_eq!(nes_cpu.pc, 0x1007);
        Ok(())
    }

    #[test]
    fn test_bcs_relative_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.pc = 0x1000;

        nes_cpu.status.set(StatusFlags::CARRY, true);

        nes_cpu.memory_bus.mem_write(nes_cpu.pc, 0xB0);
        nes_cpu
            .memory_bus
            .mem_write(nes_cpu.pc + 1, 0x05);

        nes_cpu.run();

        assert_eq!(nes_cpu.pc, 0x1007);

        Ok(())
    }

    #[test]
    fn test_beq_relative_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.pc = 0x1000;

        nes_cpu.status.set(StatusFlags::ZERO, true);

        nes_cpu.memory_bus.mem_write(nes_cpu.pc, 0xF0);
        nes_cpu
            .memory_bus
            .mem_write(nes_cpu.pc + 1, 0x05);

        nes_cpu.run();

        assert_eq!(nes_cpu.pc, 0x1007);

        Ok(())
    }

    #[test]
    fn test_bmi_relative_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.pc = 0x1000;

        nes_cpu.status.set(StatusFlags::NEGATIVE, true);

        nes_cpu.memory_bus.mem_write(nes_cpu.pc, 0x30);
        nes_cpu
            .memory_bus
            .mem_write(nes_cpu.pc + 1, 0x05);

        nes_cpu.run();

        assert_eq!(nes_cpu.pc, 0x1007);

        Ok(())
    }

    #[test]
    fn test_bne_relative_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.pc = 0x1000;

        nes_cpu.status.set(StatusFlags::ZERO, false);

        nes_cpu.memory_bus.mem_write(nes_cpu.pc, 0xD0);
        nes_cpu
            .memory_bus
            .mem_write(nes_cpu.pc + 1, 0x05);

        nes_cpu.run();

        assert_eq!(nes_cpu.pc, 0x1007);

        Ok(())
    }

    #[test]
    fn test_bpl_relative_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.pc = 0x1000;

        nes_cpu.status.set(StatusFlags::NEGATIVE, false);

        nes_cpu.memory_bus.mem_write(nes_cpu.pc, 0x10);
        nes_cpu
            .memory_bus
            .mem_write(nes_cpu.pc + 1, 0x05);

        nes_cpu.run();

        assert_eq!(nes_cpu.pc, 0x1007);

        Ok(())
    }

    #[test]
    fn test_bit_zero_page_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0b0100_0001;
        nes_cpu.memory_bus.mem_write(0x0010, 0b1100_0001);
        nes_cpu.memory_bus.mem_write(0x0000, 0x24);
        nes_cpu.memory_bus.mem_write(0x0001, 0x10);

        nes_cpu.run();

        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(nes_cpu.status.contains(StatusFlags::NEGATIVE));
        assert!(nes_cpu.status.contains(StatusFlags::OVERFLOW));

        Ok(())
    }

    #[test]
    fn test_bit_absolute_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0b0100_0001;
        nes_cpu.memory_bus.mem_write(0x2000, 0b0100_0000);
        nes_cpu.memory_bus.mem_write(0x0000, 0x2C);
        nes_cpu.memory_bus.mem_write(0x0001, 0x00);
        nes_cpu.memory_bus.mem_write(0x0002, 0x20);

        nes_cpu.run();

        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));
        assert!(nes_cpu.status.contains(StatusFlags::OVERFLOW));

        Ok(())
    }

    #[test]
    fn test_brk_instruction() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.pc = 0x1000;
        nes_cpu.memory_bus.mem_write(0x1000, 0x00);
        nes_cpu.memory_bus.mem_write(0xFFFE, 0x00);
        nes_cpu.memory_bus.mem_write(0xFFFF, 0x20);

        let initial_sp = nes_cpu.sp;

        nes_cpu.run();

        // Check if the correct return address was pushed onto the stack
        assert_eq!(
            nes_cpu.memory_bus.mem_read(0x0100 + initial_sp as u16),
            0x10
        );
        assert_eq!(
            nes_cpu
                .memory_bus
                .mem_read(0x0100 + (initial_sp as u16 - 1)),
            0x02
        );

        // Check if the status register was pushed onto the stack with the break flag set
        let pushed_status = nes_cpu
            .memory_bus
            .mem_read(0x0100 + (initial_sp as u16 - 2));
        assert_eq!(
            pushed_status & (StatusFlags::BREAK.bits() | StatusFlags::UNUSED.bits()),
            StatusFlags::BREAK.bits() | StatusFlags::UNUSED.bits()
        );

        assert!(nes_cpu.status.contains(StatusFlags::INTERRUPT));

        Ok(())
    }

    #[test]
    fn test_cmp_immediate_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x10;

        nes_cpu.memory_bus.mem_write(0x0000, 0xC9);
        nes_cpu.memory_bus.mem_write(0x0001, 0x08);
        nes_cpu.run();
        assert!(nes_cpu.status.contains(StatusFlags::CARRY));
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));

        // Compare A with immediate value that is equal
        nes_cpu.pc = 0x0002;
        nes_cpu.memory_bus.mem_write(0x0002, 0xC9);
        nes_cpu.memory_bus.mem_write(0x0003, 0x10);
        nes_cpu.run();
        assert!(nes_cpu.status.contains(StatusFlags::CARRY));
        assert!(nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));

        // Compare A with immediate value that is greater
        nes_cpu.pc = 0x0004;
        nes_cpu.memory_bus.mem_write(0x0004, 0xC9);
        nes_cpu.memory_bus.mem_write(0x0005, 0x20);
        nes_cpu.run();
        assert!(!nes_cpu.status.contains(StatusFlags::CARRY));
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(nes_cpu.status.contains(StatusFlags::NEGATIVE)); // Result is negative

        Ok(())
    }

    #[test]
    fn test_cpx_immediate_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.x = 0x10;

        nes_cpu.memory_bus.mem_write(0x0000, 0xE0);
        nes_cpu.memory_bus.mem_write(0x0001, 0x08);
        nes_cpu.run();
        assert!(nes_cpu.status.contains(StatusFlags::CARRY));
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));

        nes_cpu.pc = 0x0002;
        nes_cpu.memory_bus.mem_write(0x0002, 0xE0);
        nes_cpu.memory_bus.mem_write(0x0003, 0x10);
        nes_cpu.run();
        assert!(nes_cpu.status.contains(StatusFlags::CARRY));
        assert!(nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));

        nes_cpu.pc = 0x0004;
        nes_cpu.memory_bus.mem_write(0x0004, 0xE0);
        nes_cpu.memory_bus.mem_write(0x0005, 0x20);
        nes_cpu.run();
        assert!(!nes_cpu.status.contains(StatusFlags::CARRY));
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(nes_cpu.status.contains(StatusFlags::NEGATIVE));
        Ok(())
    }

    #[test]
    fn test_cpy_immediate_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.y = 0x10;

        nes_cpu.memory_bus.mem_write(0x0000, 0xC0);
        nes_cpu.memory_bus.mem_write(0x0001, 0x08);
        nes_cpu.run();
        assert!(nes_cpu.status.contains(StatusFlags::CARRY));
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));

        nes_cpu.pc = 0x0002;
        nes_cpu.memory_bus.mem_write(0x0002, 0xC0);
        nes_cpu.memory_bus.mem_write(0x0003, 0x10);
        nes_cpu.run();
        assert!(nes_cpu.status.contains(StatusFlags::CARRY));
        assert!(nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));

        nes_cpu.pc = 0x0004;
        nes_cpu.memory_bus.mem_write(0x0004, 0xC0);
        nes_cpu.memory_bus.mem_write(0x0005, 0x20);
        nes_cpu.run();
        assert!(!nes_cpu.status.contains(StatusFlags::CARRY));
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(nes_cpu.status.contains(StatusFlags::NEGATIVE));

        Ok(())
    }

    #[test]
    fn test_dec_zero_page_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.memory_bus.mem_write(0x00, 0xC6); // DEC ZeroPage opcode
        nes_cpu.memory_bus.mem_write(0x01, 0x10); // ZeroPage address
        nes_cpu.memory_bus.mem_write(0x10, 0x03); // Value at address 0x10
        nes_cpu.run();
        assert_eq!(nes_cpu.memory_bus.mem_read(0x10), 0x02);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));

        Ok(())
    }

    #[test]
    fn test_dec_zero_page_x_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.x = 0x01;
        nes_cpu.memory_bus.mem_write(0x00, 0xD6); // DEC ZeroPageX opcode
        nes_cpu.memory_bus.mem_write(0x01, 0x10); // ZeroPage base address
        nes_cpu.memory_bus.mem_write(0x11, 0x01); // Value at address 0x11 (0x10 + X)
        nes_cpu.run();
        assert_eq!(nes_cpu.memory_bus.mem_read(0x11), 0x00);
        assert!(nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));

        Ok(())
    }

    #[test]
    fn test_dec_absolute_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.memory_bus.mem_write(0x00, 0xCE); // DEC Absolute opcode
        nes_cpu.memory_bus.mem_write(0x01, 0x00); // Low byte of address
        nes_cpu.memory_bus.mem_write(0x02, 0x01); // High byte of address
        nes_cpu.memory_bus.mem_write(0x0100, 0x00); // Value at address 0x2000
        nes_cpu.run();
        assert_eq!(nes_cpu.memory_bus.mem_read(0x0100), 0xFF);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(nes_cpu.status.contains(StatusFlags::NEGATIVE));
        Ok(())
    }

    #[test]
    fn test_dec_absolute_x_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.x = 0x01;
        nes_cpu.memory_bus.mem_write(0x00, 0xDE); // DEC AbsoluteX opcode
        nes_cpu.memory_bus.mem_write(0x01, 0x00); // Low byte of address
        nes_cpu.memory_bus.mem_write(0x02, 0x01); // High byte of address
        nes_cpu.memory_bus.mem_write(0x101, 0x01); // Value at address 0x2001 (0x2000 + X)
        nes_cpu.run();
        assert_eq!(nes_cpu.memory_bus.mem_read(0x0101), 0x00);
        assert!(nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));
        Ok(())
    }

    #[test]
    fn test_dex() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.x = 0x01;
        nes_cpu.memory_bus.mem_write(0x00, 0xCA);
        nes_cpu.run();
        assert_eq!(nes_cpu.x, 0x00);
        assert!(nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));
        Ok(())
    }

    #[test]
    fn test_dey() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.y = 0x01;
        nes_cpu.memory_bus.mem_write(0x00, 0x88);
        nes_cpu.run();
        assert_eq!(nes_cpu.y, 0x00);
        assert!(nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));
        Ok(())
    }

    #[test]
    fn test_inc_zero_page_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.memory_bus.mem_write(0x00, 0xE6); // INC ZeroPage opcode
        nes_cpu.memory_bus.mem_write(0x01, 0x10); // ZeroPage address
        nes_cpu.memory_bus.mem_write(0x10, 0x03); // Value at address 0x10
        nes_cpu.run();
        assert_eq!(nes_cpu.memory_bus.mem_read(0x10), 0x04);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));
        Ok(())
    }

    #[test]
    fn test_inc_zero_page_x_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.x = 0x01;
        nes_cpu.memory_bus.mem_write(0x00, 0xF6); // INC ZeroPageX opcode
        nes_cpu.memory_bus.mem_write(0x01, 0x10); // ZeroPage base address
        nes_cpu.memory_bus.mem_write(0x11, 0x01); // Value at address 0x11 (0x10 + X)
        nes_cpu.run();
        assert_eq!(nes_cpu.memory_bus.mem_read(0x11), 0x02);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));
        Ok(())
    }

    #[test]
    fn test_inc_absolute_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.memory_bus.mem_write(0x00, 0xEE); // INC Absolute opcode
        nes_cpu.memory_bus.mem_write(0x01, 0x00); // Low byte of address
        nes_cpu.memory_bus.mem_write(0x02, 0x20); // High byte of address
        nes_cpu.memory_bus.mem_write(0x2000, 0x00); // Value at address 0x2000
        nes_cpu.run();
        assert_eq!(nes_cpu.memory_bus.mem_read(0x2000), 0x01);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));
        Ok(())
    }

    #[test]
    fn test_inc_absolute_x_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.x = 0x01;
        nes_cpu.memory_bus.mem_write(0x00, 0xFE); // INC AbsoluteX opcode
        nes_cpu.memory_bus.mem_write(0x01, 0x00); // Low byte of address
        nes_cpu.memory_bus.mem_write(0x02, 0x01); // High byte of address
        nes_cpu.memory_bus.mem_write(0x0101, 0x01); // Value at address 0x2001 (0x2000 + X)
        nes_cpu.run();
        assert_eq!(nes_cpu.memory_bus.mem_read(0x0101), 0x02);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));
        Ok(())
    }

    #[test]
    fn test_inx() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.x = 0x01;
        nes_cpu.memory_bus.mem_write(0x00, 0xE8);
        nes_cpu.run();
        assert_eq!(nes_cpu.x, 0x02);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));
        Ok(())
    }

    #[test]
    fn test_iny() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.y = 0x01;
        nes_cpu.memory_bus.mem_write(0x00, 0xC8);
        nes_cpu.run();
        assert_eq!(nes_cpu.y, 0x02);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));
        Ok(())
    }

    #[test]
    fn test_bvc() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.pc = 0x1000;
        nes_cpu.status.set(StatusFlags::OVERFLOW, false);
        nes_cpu.memory_bus.mem_write(0x1000, 0x50); // BVC opcode
        nes_cpu.memory_bus.mem_write(0x1001, 0x05); // Offset
        nes_cpu.run();
        assert_eq!(nes_cpu.pc, 0x1007);
        Ok(())
    }

    #[test]
    fn test_bvs() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.pc = 0x1000;
        nes_cpu.status.set(StatusFlags::OVERFLOW, true);
        nes_cpu.memory_bus.mem_write(0x1000, 0x70); // BVS opcode
        nes_cpu.memory_bus.mem_write(0x1001, 0x05); // Offset
        nes_cpu.run();
        assert_eq!(nes_cpu.pc, 0x1007);
        Ok(())
    }

    #[test]
    fn test_clc() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.status.insert(StatusFlags::CARRY);
        nes_cpu.memory_bus.mem_write(0x0000, 0x18); // CLC opcode
        nes_cpu.run();
        assert!(!nes_cpu.status.contains(StatusFlags::CARRY));
        Ok(())
    }

    #[test]
    fn test_cld() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.status.insert(StatusFlags::DECIMAL);
        nes_cpu.memory_bus.mem_write(0x0000, 0xD8); // CLD opcode
        nes_cpu.run();
        assert!(!nes_cpu.status.contains(StatusFlags::DECIMAL));
        Ok(())
    }

    #[test]
    fn test_cli() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.status.insert(StatusFlags::INTERRUPT);
        nes_cpu.memory_bus.mem_write(0x0000, 0x58); // CLI opcode
        nes_cpu.run();
        assert!(!nes_cpu.status.contains(StatusFlags::INTERRUPT));
        Ok(())
    }

    #[test]
    fn test_clv() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.status.insert(StatusFlags::OVERFLOW);
        nes_cpu.memory_bus.mem_write(0x0000, 0xB8); // CLV opcode
        nes_cpu.run();
        assert!(!nes_cpu.status.contains(StatusFlags::OVERFLOW));
        Ok(())
    }

    #[test]
    fn test_jmp_absolute_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.memory_bus.mem_write(0x0000, 0x4C); // JMP Absolute opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x00); // Low byte of address
        nes_cpu.memory_bus.mem_write(0x0002, 0x20); // High byte of address
        nes_cpu.run();
        assert_eq!(nes_cpu.pc, 0x2000);
        Ok(())
    }

    #[test]
    fn test_jmp_indirect_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.pc = 0x0000;
        nes_cpu.memory_bus.mem_write(0x0000, 0x6C); // JMP Indirect opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x00); // Low byte of indirect address
        nes_cpu.memory_bus.mem_write(0x0002, 0x01); // High byte of indirect address
        nes_cpu.memory_bus.mem_write(0x0100, 0x00); // Low byte of target address
        nes_cpu.memory_bus.mem_write(0x0101, 0x30); // High byte of target address
        nes_cpu.run();
        assert_eq!(nes_cpu.pc, 0x3000);
        Ok(())
    }

    #[test]
    fn test_jsr() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.pc = 0x1000;
        nes_cpu.memory_bus.mem_write(0x1000, 0x20); // JSR opcode
        nes_cpu.memory_bus.mem_write(0x1001, 0x00); // Low byte of address
        nes_cpu.memory_bus.mem_write(0x1002, 0x20); // High byte of address

        let initial_sp = nes_cpu.sp;

        nes_cpu.run();

        // Check if PC is set to the target address
        assert_eq!(nes_cpu.pc, 0x2000);

        // Check if the correct return address was pushed onto the stack
        assert_eq!(
            nes_cpu.memory_bus.mem_read(0x0100 + (initial_sp as u16)),
            0x10
        ); // PC high byte
        assert_eq!(
            nes_cpu
                .memory_bus
                .mem_read(0x0100 + (initial_sp as u16 - 1)),
            0x02
        ); // PC low byte + 1

        Ok(())
    }

    #[test]
    fn test_lsr_accumulator_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x02;
        nes_cpu.memory_bus.mem_write(0x0000, 0x4A); // LSR Accumulator opcode
        nes_cpu.run();
        assert_eq!(nes_cpu.a, 0x01);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::CARRY));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));

        Ok(())
    }

    #[test]
    fn test_lsr_zero_page_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.memory_bus.mem_write(0x0000, 0x46); // LSR ZeroPage opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x10); // ZeroPage address
        nes_cpu.memory_bus.mem_write(0x0010, 0x04); // Value at address 0x0010
        nes_cpu.run();
        assert_eq!(nes_cpu.memory_bus.mem_read(0x0010), 0x02);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::CARRY));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));

        Ok(())
    }

    #[test]
    fn test_lsr_zero_page_x_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.x = 0x01;
        nes_cpu.memory_bus.mem_write(0x0000, 0x56); // LSR ZeroPageX opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x10); // ZeroPage base address
        nes_cpu.memory_bus.mem_write(0x0011, 0x08); // Value at address 0x0010 + X
        nes_cpu.run();
        assert_eq!(nes_cpu.memory_bus.mem_read(0x0011), 0x04);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::CARRY));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));

        Ok(())
    }

    #[test]
    fn test_lsr_absolute_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.memory_bus.mem_write(0x0000, 0x4E); // LSR Absolute opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x00); // Low byte of address
        nes_cpu.memory_bus.mem_write(0x0002, 0x01); // High byte of address
        nes_cpu.memory_bus.mem_write(0x0100, 0x10); // Value at address 0x2000
        nes_cpu.run();
        assert_eq!(nes_cpu.memory_bus.mem_read(0x2000), 0x08);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::CARRY));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));

        Ok(())
    }

    #[test]
    fn test_lsr_absolute_x_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.x = 0x01;
        nes_cpu.memory_bus.mem_write(0x0000, 0x5E); // LSR AbsoluteX opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x00); // Low byte of address
        nes_cpu.memory_bus.mem_write(0x0002, 0x01); // High byte of address
        nes_cpu.memory_bus.mem_write(0x0101, 0x20); // Value at address 0x2000 + X
        nes_cpu.run();
        assert_eq!(nes_cpu.memory_bus.mem_read(0x0101), 0x10);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::CARRY));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));

        Ok(())
    }

    #[test]
    fn test_nop() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.pc = 0x1000;
        nes_cpu.memory_bus.mem_write(0x1000, 0xEA); // NOP opcode
        let initial_pc = nes_cpu.pc;

        nes_cpu.run();

        assert_eq!(nes_cpu.pc, initial_pc + 1); // PC should advance by 1

        Ok(())
    }

    #[test]
    fn test_ora_immediate_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x55;
        nes_cpu.memory_bus.mem_write(0x0000, 0x09); // ORA Immediate opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x0F); // Immediate value
        nes_cpu.run();
        assert_eq!(nes_cpu.a, 0x5F);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));

        Ok(())
    }

    #[test]
    fn test_ora_zero_page_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x55;
        nes_cpu.memory_bus.mem_write(0x0000, 0x05); // ORA ZeroPage opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x10); // ZeroPage address
        nes_cpu.memory_bus.mem_write(0x0010, 0x0F); // Value at address 0x0010
        nes_cpu.run();
        assert_eq!(nes_cpu.a, 0x5F);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));

        Ok(())
    }

    #[test]
    fn test_ora_zero_page_x_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x55;
        nes_cpu.x = 0x01;
        nes_cpu.memory_bus.mem_write(0x0000, 0x15); // ORA ZeroPageX opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x10); // ZeroPage base address
        nes_cpu.memory_bus.mem_write(0x0011, 0x0F); // Value at address 0x0010 + X
        nes_cpu.run();
        assert_eq!(nes_cpu.a, 0x5F);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));

        Ok(())
    }

    #[test]
    fn test_ora_absolute_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x55;
        nes_cpu.memory_bus.mem_write(0x0000, 0x0D); // ORA Absolute opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x00); // Low byte of address
        nes_cpu.memory_bus.mem_write(0x0002, 0x20); // High byte of address
        nes_cpu.memory_bus.mem_write(0x2000, 0x0F); // Value at address 0x2000
        nes_cpu.run();
        assert_eq!(nes_cpu.a, 0x5F);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));

        Ok(())
    }

    #[test]
    fn test_ora_absolute_x_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x55;
        nes_cpu.x = 0x01;
        nes_cpu.memory_bus.mem_write(0x0000, 0x1D); // ORA AbsoluteX opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x00); // Low byte of address
        nes_cpu.memory_bus.mem_write(0x0002, 0x01); // High byte of address
        nes_cpu.memory_bus.mem_write(0x0101, 0x0F); // Value at address 0x2000 + X
        nes_cpu.run();
        assert_eq!(nes_cpu.a, 0x5F);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));

        Ok(())
    }

    #[test]
    fn test_ora_absolute_y_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x55;
        nes_cpu.y = 0x01;
        nes_cpu.memory_bus.mem_write(0x0000, 0x19); // ORA AbsoluteY opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x00); // Low byte of address
        nes_cpu.memory_bus.mem_write(0x0002, 0x01); // High byte of address
        nes_cpu.memory_bus.mem_write(0x0101, 0x0F); // Value at address 0x2000 + Y
        nes_cpu.run();
        assert_eq!(nes_cpu.a, 0x5F);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));

        Ok(())
    }

    #[test]
    fn test_ora_indirect_x_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x55;
        nes_cpu.x = 0x04;
        nes_cpu.memory_bus.mem_write(0x0000, 0x01); // ORA IndirectX opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x02); // Base address
        nes_cpu.memory_bus.mem_write(0x0006, 0x00); // Low byte of effective address
        nes_cpu.memory_bus.mem_write(0x0007, 0x20); // High byte of effective address
        nes_cpu.memory_bus.mem_write(0x2000, 0x0F); // Value at effective address
        nes_cpu.run();
        assert_eq!(nes_cpu.a, 0x5F);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));

        Ok(())
    }

    #[test]
    fn test_ora_indirect_y_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x55;
        nes_cpu.y = 0x04;
        nes_cpu.memory_bus.mem_write(0x0000, 0x11); // ORA IndirectY opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x02); // Base address
        nes_cpu.memory_bus.mem_write(0x0002, 0x00); // Low byte of indirect address
        nes_cpu.memory_bus.mem_write(0x0003, 0x01); // High byte of indirect address
        nes_cpu.memory_bus.mem_write(0x0104, 0x0F); // Value at effective address (0x2000 + Y)
        nes_cpu.run();
        assert_eq!(nes_cpu.a, 0x5F);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));

        Ok(())
    }

    #[test]
    fn test_pha() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x42;
        nes_cpu.memory_bus.mem_write(0x0000, 0x48); // PHA opcode
        let initial_sp = nes_cpu.sp;
        nes_cpu.run();
        assert_eq!(
            nes_cpu.memory_bus.mem_read(0x0100 + initial_sp as u16),
            0x42
        );
        Ok(())
    }

    #[test]
    fn test_php() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.status.insert(StatusFlags::CARRY);
        nes_cpu.memory_bus.mem_write(0x0000, 0x08); // PHP opcode
        let initial_sp = nes_cpu.sp;
        nes_cpu.run();
        assert_eq!(
            nes_cpu.memory_bus.mem_read(0x0100 + initial_sp as u16),
            StatusFlags::CARRY.bits() | 0b00110000
        );
        Ok(())
    }

    #[test]
    fn test_pla() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.sp = 0xFC;
        nes_cpu.memory_bus.mem_write(0x01FD, 0x42); // Value to be pulled
        nes_cpu.memory_bus.mem_write(0x0000, 0x68); // PLA opcode
        nes_cpu.run();
        assert_eq!(nes_cpu.a, 0x42);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));
        Ok(())
    }

    #[test]
    fn test_plp() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.sp = 0xFC;
        nes_cpu
            .memory_bus
            .mem_write(0x01FD, StatusFlags::CARRY.bits() | 0b00110000); // Status to be pulled
        nes_cpu.memory_bus.mem_write(0x0000, 0x28); // PLP opcode
        nes_cpu.run();
        assert!(nes_cpu.status.contains(StatusFlags::CARRY));
        assert!(!nes_cpu.status.contains(StatusFlags::BREAK));
        assert!(!nes_cpu.status.contains(StatusFlags::UNUSED));
        Ok(())
    }

    #[test]
    fn test_ror_accumulator_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x01;
        nes_cpu.memory_bus.mem_write(0x0000, 0x6A); // ROR Accumulator opcode
        nes_cpu.status.set(StatusFlags::CARRY, true); // Set initial carry flag
        nes_cpu.run();
        assert_eq!(nes_cpu.a, 0x80);
        assert!(nes_cpu.status.contains(StatusFlags::CARRY));
        assert!(nes_cpu.status.contains(StatusFlags::NEGATIVE));
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));

        Ok(())
    }

    #[test]
    fn test_ror_zero_page_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.memory_bus.mem_write(0x0000, 0x66); // ROR ZeroPage opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x10); // ZeroPage address
        nes_cpu.memory_bus.mem_write(0x0010, 0x02); // Value at address 0x0010
        nes_cpu.status.set(StatusFlags::CARRY, true); // Set initial carry flag
        nes_cpu.run(); // Run the CPU
        assert_eq!(nes_cpu.memory_bus.mem_read(0x0010), 0x81);
        assert!(!nes_cpu.status.contains(StatusFlags::CARRY));
        assert!(nes_cpu.status.contains(StatusFlags::NEGATIVE));
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        Ok(())
    }

    #[test]
    fn test_ror_zero_page_x_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.x = 0x01;
        nes_cpu.memory_bus.mem_write(0x0000, 0x76); // ROR ZeroPageX opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x10); // ZeroPage base address
        nes_cpu.memory_bus.mem_write(0x0011, 0x04); // Value at address 0x0010 + X
        nes_cpu.status.set(StatusFlags::CARRY, false); // Clear initial carry flag
        nes_cpu.run(); // Run the CPU
        assert_eq!(nes_cpu.memory_bus.mem_read(0x0011), 0x02);
        assert!(!nes_cpu.status.contains(StatusFlags::CARRY));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        Ok(())
    }

    #[test]
    fn test_ror_absolute_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.memory_bus.mem_write(0x0000, 0x6E); // ROR Absolute opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x00); // Low byte of address
        nes_cpu.memory_bus.mem_write(0x0002, 0x20); // High byte of address
        nes_cpu.memory_bus.mem_write(0x2000, 0x08); // Value at address 0x2000
        nes_cpu.status.set(StatusFlags::CARRY, true); // Set initial carry flag
        nes_cpu.run(); // Run the CPU
        assert_eq!(nes_cpu.memory_bus.mem_read(0x2000), 0x84);
        assert!(!nes_cpu.status.contains(StatusFlags::CARRY));
        assert!(nes_cpu.status.contains(StatusFlags::NEGATIVE));
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        Ok(())
    }

    #[test]
    fn test_ror_absolute_x_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.x = 0x01;
        nes_cpu.memory_bus.mem_write(0x0000, 0x7E); // ROR AbsoluteX opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x00); // Low byte of address
        nes_cpu.memory_bus.mem_write(0x0002, 0x01); // High byte of address
        nes_cpu.memory_bus.mem_write(0x0101, 0x10); // Value at address 0x2000 + X
        nes_cpu.status.set(StatusFlags::CARRY, false); // Clear initial carry flag
        nes_cpu.run(); // Run the CPU
        assert_eq!(nes_cpu.memory_bus.mem_read(0x0101), 0x08);
        assert!(!nes_cpu.status.contains(StatusFlags::CARRY));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        Ok(())
    }

    #[test]
    fn test_rol_accumulator_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x80;
        nes_cpu.memory_bus.mem_write(0x0000, 0x2A); // ROL Accumulator opcode
        nes_cpu.status.set(StatusFlags::CARRY, true); // Set initial carry flag
        nes_cpu.run(); // Run the CPU
        assert_eq!(nes_cpu.a, 0x01);
        assert!(nes_cpu.status.contains(StatusFlags::CARRY));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        Ok(())
    }

    #[test]
    fn test_rol_zero_page_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.memory_bus.mem_write(0x0000, 0x26); // ROL ZeroPage opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x10); // ZeroPage address
        nes_cpu.memory_bus.mem_write(0x0010, 0x40); // Value at address 0x0010
        nes_cpu.status.set(StatusFlags::CARRY, true); // Set initial carry flag
        nes_cpu.run(); // Run the CPU
        assert_eq!(nes_cpu.memory_bus.mem_read(0x0010), 0x81);
        assert!(!nes_cpu.status.contains(StatusFlags::CARRY));
        assert!(nes_cpu.status.contains(StatusFlags::NEGATIVE));
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        Ok(())
    }

    #[test]
    fn test_rol_zero_page_x_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.x = 0x01;
        nes_cpu.memory_bus.mem_write(0x0000, 0x36); // ROL ZeroPageX opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x10); // ZeroPage base address
        nes_cpu.memory_bus.mem_write(0x0011, 0x20); // Value at address 0x0010 + X
        nes_cpu.status.set(StatusFlags::CARRY, false); // Clear initial carry flag
        nes_cpu.run(); // Run the CPU
        assert_eq!(nes_cpu.memory_bus.mem_read(0x0011), 0x40);
        assert!(!nes_cpu.status.contains(StatusFlags::CARRY));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        Ok(())
    }

    #[test]
    fn test_rol_absolute_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.memory_bus.mem_write(0x0000, 0x2E); // ROL Absolute opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x00); // Low byte of address
        nes_cpu.memory_bus.mem_write(0x0002, 0x20); // High byte of address
        nes_cpu.memory_bus.mem_write(0x2000, 0x80); // Value at address 0x2000
        nes_cpu.status.set(StatusFlags::CARRY, true); // Set initial carry flag
        nes_cpu.run(); // Run the CPU
        assert_eq!(nes_cpu.memory_bus.mem_read(0x2000), 0x01);
        assert!(nes_cpu.status.contains(StatusFlags::CARRY));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        Ok(())
    }

    #[test]
    fn test_rol_absolute_x_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.x = 0x01;
        nes_cpu.memory_bus.mem_write(0x0000, 0x3E); // ROL AbsoluteX opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x00); // Low byte of address
        nes_cpu.memory_bus.mem_write(0x0002, 0x01); // High byte of address
        nes_cpu.memory_bus.mem_write(0x0101, 0x01); // Value at address 0x2000 + X
        nes_cpu.status.set(StatusFlags::CARRY, false); // Clear initial carry flag
        nes_cpu.run();
        assert_eq!(nes_cpu.memory_bus.mem_read(0x0101), 0x02);
        assert!(!nes_cpu.status.contains(StatusFlags::CARRY));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));

        Ok(())
    }

    #[test]
    fn test_rti() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );

        // Set up stack and memory
        nes_cpu.memory_bus.mem_write(0x0000, 0x40); // RTI opcode
        nes_cpu.sp = 0xFC;
        nes_cpu.memory_bus.mem_write(
            0x01FD,
            StatusFlags::CARRY.bits() | StatusFlags::INTERRUPT.bits(),
        ); // Pushed status register
        nes_cpu.memory_bus.mem_write(0x01FE, 0x34); // Pushed PC low byte
        nes_cpu.memory_bus.mem_write(0x01FF, 0x12); // Pushed PC high byte

        nes_cpu.run();

        assert_eq!(nes_cpu.pc, 0x1234);
        assert!(nes_cpu.status.contains(StatusFlags::CARRY));
        assert!(nes_cpu.status.contains(StatusFlags::INTERRUPT));
        assert_eq!(nes_cpu.sp, 0xFF);

        Ok(())
    }

    #[test]
    fn test_rts() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );

        // Set up stack and memory
        nes_cpu.memory_bus.mem_write(0x0000, 0x60); // RTS opcode
        nes_cpu.memory_bus.mem_write(0x01FE, 0x34); // Pushed PC low byte
        nes_cpu.memory_bus.mem_write(0x01FF, 0x12); // Pushed PC high byte

        nes_cpu.run();

        assert_eq!(nes_cpu.pc, 0x1235); // PC + 1
        assert_eq!(nes_cpu.sp, 0xFF);

        Ok(())
    }

    #[test]
    fn test_sbc_immediate_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x10;
        nes_cpu.memory_bus.mem_write(0x0000, 0xE9); // SBC Immediate opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x05); // Operand
        nes_cpu.status.set(StatusFlags::CARRY, true); // No borrow
        nes_cpu.run();
        assert_eq!(nes_cpu.a, 0x0B);
        assert!(nes_cpu.status.contains(StatusFlags::CARRY)); // No borrow
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));
        assert!(!nes_cpu.status.contains(StatusFlags::OVERFLOW));
        Ok(())
    }

    #[test]
    fn test_sbc_zero_page_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x20;
        nes_cpu.memory_bus.mem_write(0x0000, 0xE5); // SBC ZeroPage opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x10); // ZeroPage address
        nes_cpu.memory_bus.mem_write(0x0010, 0x10); // Value at address 0x0010
        nes_cpu.status.set(StatusFlags::CARRY, true); // No borrow
        nes_cpu.run();
        assert_eq!(nes_cpu.a, 0x10);
        assert!(nes_cpu.status.contains(StatusFlags::CARRY)); // No borrow
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));
        assert!(!nes_cpu.status.contains(StatusFlags::OVERFLOW));
        Ok(())
    }

    #[test]
    fn test_sec() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.status.set(StatusFlags::CARRY, false);
        nes_cpu.memory_bus.mem_write(0x0000, 0x38);
        nes_cpu.run();
        assert!(nes_cpu.status.contains(StatusFlags::CARRY));

        Ok(())
    }

    #[test]
    fn test_sed() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.status.set(StatusFlags::DECIMAL, false);
        nes_cpu.memory_bus.mem_write(0x0000, 0xF8);
        nes_cpu.run();
        assert!(nes_cpu.status.contains(StatusFlags::DECIMAL));

        Ok(())
    }

    #[test]
    fn test_sei() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.status.set(StatusFlags::INTERRUPT, false);
        nes_cpu.memory_bus.mem_write(0x0000, 0x78);
        nes_cpu.run();
        assert!(nes_cpu.status.contains(StatusFlags::INTERRUPT));

        Ok(())
    }

    #[test]
    fn test_sta_zero_page_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x42;
        nes_cpu.memory_bus.mem_write(0x0000, 0x85); // STA ZeroPage opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x10); // ZeroPage address
        nes_cpu.run();
        assert_eq!(nes_cpu.memory_bus.mem_read(0x0010), 0x42);

        Ok(())
    }

    #[test]
    fn test_sta_zero_page_x_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x42;
        nes_cpu.x = 0x01;
        nes_cpu.memory_bus.mem_write(0x0000, 0x95); // STA ZeroPageX opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x10); // ZeroPage base address
        nes_cpu.run();
        assert_eq!(nes_cpu.memory_bus.mem_read(0x0011), 0x42);

        Ok(())
    }

    #[test]
    fn test_sta_absolute_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x42;
        nes_cpu.memory_bus.mem_write(0x0000, 0x8D); // STA Absolute opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x00); // Low byte of address
        nes_cpu.memory_bus.mem_write(0x0002, 0x01); // High byte of address
        nes_cpu.run();
        assert_eq!(nes_cpu.memory_bus.mem_read(0x0100), 0x42);

        Ok(())
    }

    #[test]
    fn test_sta_absolute_x_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x42;
        nes_cpu.x = 0x01;
        nes_cpu.memory_bus.mem_write(0x0000, 0x9D); // STA AbsoluteX opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x00); // Low byte of address
        nes_cpu.memory_bus.mem_write(0x0002, 0x01); // High byte of address
        nes_cpu.run();
        assert_eq!(nes_cpu.memory_bus.mem_read(0x0101), 0x42);
        Ok(())
    }

    #[test]
    fn test_sta_absolute_y_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x42;
        nes_cpu.y = 0x01;
        nes_cpu.memory_bus.mem_write(0x0000, 0x99); // STA AbsoluteY opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x00); // Low byte of address
        nes_cpu.memory_bus.mem_write(0x0002, 0x01); // High byte of address
        nes_cpu.run();
        assert_eq!(nes_cpu.memory_bus.mem_read(0x0101), 0x42);

        Ok(())
    }

    #[test]
    fn test_sta_indirect_x_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x42;
        nes_cpu.x = 0x04;
        nes_cpu.memory_bus.mem_write(0x0000, 0x81); // STA IndirectX opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x02); // Base address
        nes_cpu.memory_bus.mem_write(0x0006, 0x00); // Low byte of target address
        nes_cpu.memory_bus.mem_write(0x0007, 0x20); // High byte of target address
        nes_cpu.run();
        assert_eq!(nes_cpu.memory_bus.mem_read(0x2000), 0x42);

        Ok(())
    }

    #[test]
    fn test_sta_indirect_y_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x42;
        nes_cpu.y = 0x04;
        nes_cpu.memory_bus.mem_write(0x0000, 0x91); // STA IndirectY opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x02); // Base address
        nes_cpu.memory_bus.mem_write(0x0002, 0x00); // Low byte of target address
        nes_cpu.memory_bus.mem_write(0x0003, 0x01); // High byte of target address
        nes_cpu.run();
        assert_eq!(nes_cpu.memory_bus.mem_read(0x0104), 0x42);

        Ok(())
    }

    #[test]
    fn test_stx_zero_page_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.x = 0x42;
        nes_cpu.memory_bus.mem_write(0x0000, 0x86); // STX ZeroPage opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x10); // ZeroPage address
        nes_cpu.run();
        assert_eq!(nes_cpu.memory_bus.mem_read(0x0010), 0x42);

        Ok(())
    }

    #[test]
    fn test_stx_zero_page_y_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.x = 0x42;
        nes_cpu.y = 0x01;
        nes_cpu.memory_bus.mem_write(0x0000, 0x96); // STX ZeroPageY opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x10); // ZeroPage base address
        nes_cpu.run();
        assert_eq!(nes_cpu.memory_bus.mem_read(0x0011), 0x42);

        Ok(())
    }

    #[test]
    fn test_stx_absolute_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.x = 0x42;
        nes_cpu.memory_bus.mem_write(0x0000, 0x8E); // STX Absolute opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x00); // Low byte of address
        nes_cpu.memory_bus.mem_write(0x0002, 0x20); // High byte of address
        nes_cpu.run();
        assert_eq!(nes_cpu.memory_bus.mem_read(0x2000), 0x42);

        Ok(())
    }

    #[test]
    fn test_sty_zero_page_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.y = 0x42;
        nes_cpu.memory_bus.mem_write(0x0000, 0x84); // STY ZeroPage opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x10); // ZeroPage address
        nes_cpu.run();
        assert_eq!(nes_cpu.memory_bus.mem_read(0x0010), 0x42);

        Ok(())
    }

    #[test]
    fn test_sty_zero_page_x_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.y = 0x42;
        nes_cpu.x = 0x01;
        nes_cpu.memory_bus.mem_write(0x0000, 0x94); // STY ZeroPageX opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x10); // ZeroPage base address
        nes_cpu.run();
        assert_eq!(nes_cpu.memory_bus.mem_read(0x0011), 0x42);

        Ok(())
    }

    #[test]
    fn test_sty_absolute_mode() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.y = 0x42;
        nes_cpu.memory_bus.mem_write(0x0000, 0x8C); // STY Absolute opcode
        nes_cpu.memory_bus.mem_write(0x0001, 0x00); // Low byte of address
        nes_cpu.memory_bus.mem_write(0x0002, 0x20); // High byte of address
        nes_cpu.run();
        assert_eq!(nes_cpu.memory_bus.mem_read(0x2000), 0x42);

        Ok(())
    }

    #[test]
    fn test_tax() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x42;
        nes_cpu.memory_bus.mem_write(0x0000, 0xAA); // TAX opcode
        nes_cpu.run();
        assert_eq!(nes_cpu.x, 0x42);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));

        Ok(())
    }

    #[test]
    fn test_tay() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.a = 0x42;
        nes_cpu.memory_bus.mem_write(0x0000, 0xA8); // TAY opcode
        nes_cpu.run();
        assert_eq!(nes_cpu.y, 0x42);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));

        Ok(())
    }

    #[test]
    fn test_tsx() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.sp = 0x42;
        nes_cpu.memory_bus.mem_write(0x0000, 0xBA); // TSX opcode
        nes_cpu.run();
        assert_eq!(nes_cpu.x, 0x42);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));

        Ok(())
    }

    #[test]
    fn test_txa() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.x = 0x42;
        nes_cpu.memory_bus.mem_write(0x0000, 0x8A); // TXA opcode
        nes_cpu.run();
        assert_eq!(nes_cpu.a, 0x42);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));

        Ok(())
    }

    #[test]
    fn test_txs() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.x = 0x42;
        nes_cpu.memory_bus.mem_write(0x0000, 0x9A); // TXS opcode
        nes_cpu.run();
        assert_eq!(nes_cpu.sp, 0x42);

        Ok(())
    }

    #[test]
    fn test_tya() -> Result<(), Box<dyn Error>> {
        let nes_file = NesFile::load("tests/snake.nes")?;
        let mut nes_cpu = CPU::new(
            nes_file.prg_rom.clone(),
            nes_file.chr_rom.clone(),
            nes_file.mirroring,
        );
        nes_cpu.y = 0x42;
        nes_cpu.memory_bus.mem_write(0x0000, 0x98); // TYA opcode
        nes_cpu.run();
        assert_eq!(nes_cpu.a, 0x42);
        assert!(!nes_cpu.status.contains(StatusFlags::ZERO));
        assert!(!nes_cpu.status.contains(StatusFlags::NEGATIVE));

        Ok(())
    }
}
