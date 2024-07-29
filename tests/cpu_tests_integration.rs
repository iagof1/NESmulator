#[cfg(test)]
mod tests {
    use nes::cpu::CPU;
    use std::fs::File;
    use std::io::{self, prelude::*};

    #[test]
    pub fn test_run_simple_program() -> io::Result<()> {
        let mut nes_cpu = CPU::new();
        let mut file = File::open("tests/example.bin")?;
        let mut contents = Vec::new();

        file.read_to_end(&mut contents)?;
        nes_cpu.memory[0x8000..0x8000 + contents.len()].copy_from_slice(&contents);
        nes_cpu.pc = 0x8000;
        nes_cpu.run_program();
        assert_eq!(nes_cpu.memory[0x0002], 30);

        Ok(())
    }
}
