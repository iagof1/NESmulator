#[cfg(test)]

mod tests {
    use nes::ppu::PPU;
    use nes::rom::Mirroring;

    #[test]
    fn test_ppu_vram_writes() {
        let mut ppu = PPU::new_empty_rom();
        ppu.registers.write_to_ppu_addr(0x23);
        ppu.registers.write_to_ppu_addr(0x05);
        ppu.write_to_data(0x66);

        assert_eq!(ppu.vram[0x0305], 0x66);
    }

    #[test]
    fn test_ppu_vram_reads() {
        let mut ppu = PPU::new_empty_rom();
        ppu.registers.write_control(0);
        ppu.vram[0x0305] = 0x66;

        ppu.registers.write_to_ppu_addr(0x23);
        ppu.registers.write_to_ppu_addr(0x05);

        ppu.read_data(); //load_into_buffer
        assert_eq!(ppu.registers.addr.get(), 0x2306);
        assert_eq!(ppu.read_data(), 0x66);
    }

    #[test]
    fn test_ppu_vram_reads_cross_page() {
        let mut ppu = PPU::new_empty_rom();
        ppu.registers.write_control(0);
        ppu.vram[0x01ff] = 0x66;
        ppu.vram[0x0200] = 0x77;

        ppu.registers.write_to_ppu_addr(0x21);
        ppu.registers.write_to_ppu_addr(0xff);

        ppu.read_data(); //load_into_buffer
        assert_eq!(ppu.read_data(), 0x66);
        assert_eq!(ppu.read_data(), 0x77);
    }

    #[test]
    fn test_ppu_vram_reads_step_32() {
        let mut ppu = PPU::new_empty_rom();
        ppu.registers.write_control(0b100);
        ppu.vram[0x01ff] = 0x66;
        ppu.vram[0x01ff + 32] = 0x77;
        ppu.vram[0x01ff + 64] = 0x88;

        ppu.registers.write_to_ppu_addr(0x21);
        ppu.registers.write_to_ppu_addr(0xff);

        ppu.read_data(); //load_into_buffer
        assert_eq!(ppu.read_data(), 0x66);
        assert_eq!(ppu.read_data(), 0x77);
        assert_eq!(ppu.read_data(), 0x88);
    }

    // Horizontal: https://wiki.nesdev.com/w/index.php/Mirroring
    //   [0x2000 A ] [0x2400 a ]
    //   [0x2800 B ] [0x2C00 b ]
    #[test]
    fn test_vram_horizontal_mirror() {
        let mut ppu = PPU::new_empty_rom();
        ppu.registers.write_to_ppu_addr(0x24);
        ppu.registers.write_to_ppu_addr(0x05);

        ppu.write_to_data(0x66); //write to a

        ppu.registers.write_to_ppu_addr(0x28);
        ppu.registers.write_to_ppu_addr(0x05);

        ppu.write_to_data(0x77); //write to B

        ppu.registers.write_to_ppu_addr(0x20);
        ppu.registers.write_to_ppu_addr(0x05);

        ppu.read_data(); //load into buffer
        assert_eq!(ppu.read_data(), 0x66); //read from A

        ppu.registers.write_to_ppu_addr(0x2C);
        ppu.registers.write_to_ppu_addr(0x05);

        ppu.read_data(); //load into buffer
        assert_eq!(ppu.read_data(), 0x77); //read from b
    }

    // Vertical: https://wiki.nesdev.com/w/index.php/Mirroring
    //   [0x2000 A ] [0x2400 B ]
    //   [0x2800 a ] [0x2C00 b ]
    #[test]
    fn test_vram_vertical_mirror() {
        let mut ppu = PPU::new(vec![0; 2048], Mirroring::Vertical);

        ppu.registers.write_to_ppu_addr(0x20);
        ppu.registers.write_to_ppu_addr(0x05);

        ppu.write_to_data(0x66); //write to A

        ppu.registers.write_to_ppu_addr(0x2C);
        ppu.registers.write_to_ppu_addr(0x05);

        ppu.write_to_data(0x77); //write to b

        ppu.registers.write_to_ppu_addr(0x28);
        ppu.registers.write_to_ppu_addr(0x05);

        ppu.read_data(); //load into buffer
        assert_eq!(ppu.read_data(), 0x66); //read from a

        ppu.registers.write_to_ppu_addr(0x24);
        ppu.registers.write_to_ppu_addr(0x05);

        ppu.read_data(); //load into buffer
        assert_eq!(ppu.read_data(), 0x77); //read from B
    }

    #[test]
    fn test_read_status_resets_latch() {
        let mut ppu = PPU::new_empty_rom();
        ppu.vram[0x0305] = 0x66;

        ppu.registers.write_to_ppu_addr(0x21);
        ppu.registers.write_to_ppu_addr(0x23);
        ppu.registers.write_to_ppu_addr(0x05);

        ppu.read_data(); //load_into_buffer
        assert_ne!(ppu.read_data(), 0x66);

        ppu.registers.read_status();

        ppu.registers.write_to_ppu_addr(0x23);
        ppu.registers.write_to_ppu_addr(0x05);

        ppu.read_data(); //load_into_buffer
        assert_eq!(ppu.read_data(), 0x66);
    }

    #[test]
    fn test_ppu_vram_mirroring() {
        let mut ppu = PPU::new_empty_rom();
        ppu.registers.write_control(0);
        ppu.vram[0x0305] = 0x66;

        ppu.registers.write_to_ppu_addr(0x63); //0x6305 -> 0x2305
        ppu.registers.write_to_ppu_addr(0x05);

        ppu.read_data(); //load into_buffer
        assert_eq!(ppu.read_data(), 0x66);
        // assert_eq!(ppu.addr.read(), 0x0306)
    }

    #[test]
    fn test_read_status_resets_vblank() {
        let mut ppu = PPU::new_empty_rom();
        ppu.registers.status.set_vblank_status(true);

        let status = ppu.registers.read_status();

        assert_eq!(status >> 7, 1);
        assert_eq!(ppu.registers.status.snapshot() >> 7, 0);
    }

    #[test]
    fn test_oam_read_write() {
        let mut ppu = PPU::new_empty_rom();
        ppu.registers.write_to_oam_addr(0x10);
        ppu.registers.write_to_oam_data(0x66);
        ppu.registers.write_to_oam_data(0x77);

        ppu.registers.write_to_oam_addr(0x10);
        assert_eq!(ppu.registers.read_oam_data(), 0x66);

        ppu.registers.write_to_oam_addr(0x11);
        assert_eq!(ppu.registers.read_oam_data(), 0x77);
    }

    #[test]
    fn test_oam_dma() {
        let mut ppu = PPU::new_empty_rom();

        let mut data = [0x66; 256];
        data[0] = 0x77;
        data[255] = 0x88;

        ppu.registers.write_to_oam_addr(0x10);
        ppu.write_oam_dma(&data);

        ppu.registers.write_to_oam_addr(0xf); //wrap around
        assert_eq!(ppu.registers.read_oam_data(), 0x88);

        ppu.registers.write_to_oam_addr(0x10);
        ppu.registers.write_to_oam_addr(0x77);
        ppu.registers.write_to_oam_addr(0x11);
        ppu.registers.write_to_oam_addr(0x66);
    }
}
