#[cfg(test)]
mod tests {
    use nes::nes_file::NesFile;
    use nes::ppu::PPU;
    use nes::ppu_ctrl_reg::CtrlRegister;
    use nes::tile::SystemPalette;
    use std::io;

    #[test]
    fn test_ppu_delayed_read_data() -> io::Result<()> {
        let nes_file = NesFile::load("tests/Super Mario Bros. (World).nes")?;
        let chr_rom = nes_file.chr_rom.clone();
        let mut ppu = PPU::new(nes_file.chr_rom, nes_file.mirroring);
        let chr_rom_length = chr_rom.len();

        ppu.set_addr(0x00);
        ppu.set_addr(0x00);

        ppu.set_ctrl(!CtrlRegister::VRAM_INC.bits());

        assert_eq!(chr_rom, ppu.chr_rom);

        let initial_internal_buf_value = ppu.read_data();

        assert_eq!(initial_internal_buf_value, 0);

        for i in 0..chr_rom_length {
            let result = ppu.read_data();
            assert_eq!(chr_rom[i], result);
        }

        Ok(())
    }

    #[test]
    fn test_system_palette() {
        let palette = SystemPalette::new();
        assert_eq!(palette.colors[0], 0x808080);
        assert_eq!(palette.colors[63], 0x111111);
    }

    #[test]
    fn test_integration() -> io::Result<()> {
        let nes_file = NesFile::load("tests/Super Mario Bros. (World).nes")?;
        let mut ppu = PPU::new(nes_file.chr_rom, nes_file.mirroring);

        // Read data and verify
        ppu.set_addr(0x20);
        ppu.set_addr(0x00);

        ppu.read_data();

        for i in 0..ppu.name_tables.len() {
            let value = ppu.read_data();
            assert_eq!(value, i as u8);
        }

        // Check palette data
        for i in 0..ppu.palette_ram.len() {
            let value = ppu.read_data();
            assert_eq!(value, (i % 64) as u8);
        }

        Ok(())
    }
}
