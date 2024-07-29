#[cfg(test)]
mod tests {
    use nes::nes_file::NesFile;
    use nes::ppu::PPU;
    use nes::ppu_ctrl_reg::CtrlRegister;
    use std::io;

    #[test]
    fn test_ppu_delayed_read_data() -> io::Result<()> {
        let nes_file = NesFile::load("tests/Super Mario Bros. (World).nes")?;
        let chr_room = nes_file.chr_rom.clone();
        let mut ppu = PPU::new(nes_file.chr_rom, nes_file.mirroring);
        let chr_rom_length = chr_room.len();

        ppu.set_addr(0x00);
        ppu.set_addr(0x00);

        ppu.set_ctrl(!CtrlRegister::VRAM_INC.bits());

        assert_eq!(chr_room, ppu.chr_rom);

        let initial_internal_buf_value = ppu.read_data();

        assert_eq!(initial_internal_buf_value, 0);

        for i in 0..chr_rom_length {
            let result = ppu.read_data();
            assert_eq!(chr_room[i], result);
        }

        Ok(())
    }
}
