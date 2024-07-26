mod cpu;
mod instructions;
mod nes_file;

use std::io;

use cpu::CPU;
use nes_file::NesFile;

fn main() -> io::Result<()> {
    let nes_file = NesFile::load("tests/Super Mario Bros. (World).nes")?;
    let mut nes_cpu = CPU::new();

    println!("PRG ROM Size: {} bytes", nes_file.prg_rom.len());
    println!("CHR ROM Size: {} bytes", nes_file.chr_rom.len());
    println!("Mapper: {}", nes_file.mapper);
    println!("Mirroring: {:?}", nes_file.mirroring);

    nes_cpu.memory[0x8000..0x8000 + nes_file.prg_rom.len()].copy_from_slice(&nes_file.prg_rom);
    nes_cpu.pc = 0x8000;
    nes_cpu.run_program();

    Ok(())
}
