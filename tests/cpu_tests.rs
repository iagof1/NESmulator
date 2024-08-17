#[cfg(test)]

mod tests {
    use nes::bus::Bus;
    use nes::cpu::StatusFlags;
    use nes::cpu::CPU;
    use nes::rom::Rom;

    #[test]
    fn test_adc_immediate_mode() {
        let bytes: Vec<u8> = std::fs::read("src/samples/Balloon Fight (USA).nes").unwrap();
        let rom = Rom::new(&bytes).unwrap();
        let bus = Bus::new(rom, |_| {});
        let mut cpu = CPU::new(bus);

        cpu.a = 0x10;
        cpu.status.insert(StatusFlags::CARRY);

        let opcode: u8 = 0x69;
        let operand: u8 = 0x20;

        cpu.bus.mem_write(cpu.pc, opcode);
        cpu.bus.mem_write(cpu.pc + 1, operand);

        cpu.step();

        assert_eq!(cpu.a, 0x10 + 0x20 + 1);
        assert!(cpu.status.contains(StatusFlags::CARRY) == false);
        assert!(cpu.status.contains(StatusFlags::OVERFLOW) == false);
    }

    #[test]
    fn test_lda_from_memory() {
        let bytes: Vec<u8> = std::fs::read("src/samples/Balloon Fight (USA).nes").unwrap();
        let rom = Rom::new(&bytes).unwrap();
        let bus = Bus::new(rom, |_| {});
        let mut cpu = CPU::new(bus);

        cpu.a = 0x10;
        cpu.status.insert(StatusFlags::CARRY);

        let opcode: u8 = 0xA9;
        let operand: u8 = 0x11;

        cpu.bus.mem_write(cpu.pc, opcode);
        cpu.bus.mem_write(cpu.pc + 1, operand);

        cpu.step();

        assert_eq!(cpu.a, 0x11);
    }
}
