#[cfg(test)]
mod tests {
    use nes::bus::Bus;
    use nes::cpu::StatusFlags;
    use nes::cpu::CPU;
    use nes::rom::Rom;

    fn fresh_cpu() -> CPU<'static> {
        let bytes: Vec<u8> = std::fs::read("src/samples/Balloon Fight (USA).nes").unwrap();
        let rom = Rom::new(&bytes).unwrap();
        let bus = Bus::new(rom, |_, _| {});
        let mut cpu = CPU::new(bus);
        cpu.pc = 0x0000;
        cpu
    }

    #[test]
    fn test_adc_immediate_mode() {
        let mut cpu = fresh_cpu();
        cpu.a = 0x10;
        cpu.status.insert(StatusFlags::CARRY);
        cpu.bus.mem_write(cpu.pc, 0x69);
        cpu.bus.mem_write(cpu.pc + 1, 0x20);
        cpu.step();
        assert_eq!(cpu.a, 0x31);
        assert!(!cpu.status.contains(StatusFlags::CARRY));
        assert!(!cpu.status.contains(StatusFlags::OVERFLOW));
    }

    #[test]
    fn test_lda_immediate() {
        let mut cpu = fresh_cpu();
        cpu.bus.mem_write(cpu.pc, 0xA9);
        cpu.bus.mem_write(cpu.pc + 1, 0x11);
        cpu.step();
        assert_eq!(cpu.a, 0x11);
    }

    #[test]
    fn test_ldx_zeropage_y_uses_y() {
        let mut cpu = fresh_cpu();
        cpu.y = 0x05;
        cpu.bus.mem_write(0x0015, 0x42);
        cpu.bus.mem_write(cpu.pc, 0xB6); // LDX zp,Y
        cpu.bus.mem_write(cpu.pc + 1, 0x10);
        cpu.step();
        assert_eq!(cpu.x, 0x42);
    }

    #[test]
    fn test_asl_accumulator_sets_zero_flag() {
        let mut cpu = fresh_cpu();
        cpu.a = 0x80;
        cpu.bus.mem_write(cpu.pc, 0x0A);
        cpu.step();
        assert_eq!(cpu.a, 0x00);
        assert!(cpu.status.contains(StatusFlags::ZERO));
        assert!(cpu.status.contains(StatusFlags::CARRY));
        assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
    }

    #[test]
    fn test_sbc_borrow_and_carry() {
        let mut cpu = fresh_cpu();
        cpu.a = 0x50;
        cpu.status.insert(StatusFlags::CARRY); // no borrow in
        cpu.bus.mem_write(cpu.pc, 0xE9);
        cpu.bus.mem_write(cpu.pc + 1, 0x30);
        cpu.step();
        assert_eq!(cpu.a, 0x20);
        assert!(cpu.status.contains(StatusFlags::CARRY)); // no borrow out
        assert!(!cpu.status.contains(StatusFlags::OVERFLOW));
    }

    #[test]
    fn test_sbc_signed_overflow() {
        let mut cpu = fresh_cpu();
        cpu.a = 0x50;
        cpu.status.insert(StatusFlags::CARRY);
        cpu.bus.mem_write(cpu.pc, 0xE9);
        cpu.bus.mem_write(cpu.pc + 1, 0xB0); // -80 -> 0x50 - (-80) = +160 overflow
        cpu.step();
        assert_eq!(cpu.a, 0xA0);
        assert!(cpu.status.contains(StatusFlags::OVERFLOW));
        assert!(!cpu.status.contains(StatusFlags::CARRY));
    }

    #[test]
    fn test_indirect_jmp_page_bug() {
        let mut cpu = fresh_cpu();
        // JMP ($02FF) -> reads lo from $02FF, hi from $0200 (not $0300)
        cpu.bus.mem_write(0x02FF, 0x34);
        cpu.bus.mem_write(0x0300, 0xFF); // would-be hi if no bug
        cpu.bus.mem_write(0x0200, 0x12); // actual hi due to 6502 bug
        cpu.bus.mem_write(cpu.pc, 0x6C);
        cpu.bus.mem_write(cpu.pc + 1, 0xFF);
        cpu.bus.mem_write(cpu.pc + 2, 0x02);
        cpu.step();
        assert_eq!(cpu.pc, 0x1234);
    }

    #[test]
    fn test_jsr_rts_roundtrip() {
        let mut cpu = fresh_cpu();
        cpu.pc = 0x0600;
        cpu.bus.mem_write(0x0600, 0x20); // JSR $0700
        cpu.bus.mem_write(0x0601, 0x00);
        cpu.bus.mem_write(0x0602, 0x07);
        cpu.bus.mem_write(0x0700, 0x60); // RTS
        cpu.step();
        assert_eq!(cpu.pc, 0x0700);
        cpu.step();
        assert_eq!(cpu.pc, 0x0603);
    }
}
