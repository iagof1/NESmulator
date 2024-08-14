const NES_TAG: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];
const PRG_ROM_PAGE_SIZE: usize = 16384;
const CHR_ROM_PAGE_SIZE: usize = 8192;

#[derive(Debug, PartialEq)]
pub enum Mirroring {
    Vertical,
    Horizontal,
    FourScreen,
}

pub struct Rom {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub mapper: u8,
    pub mirroring: Mirroring,
}

impl Rom {
    pub fn new(raw: &Vec<u8>) -> Result<Rom, String> {
        if &raw[0..4] != NES_TAG {
            return Err("Invalid NES file".to_string());
        }

        let mapper = (raw[7] & 0xF0) | (raw[6] >> 4);
        let ines_ver = (raw[7] >> 2) & 0x3;

        if ines_ver != 0 {
            return Err("Only iNES version 1.0 is supported".to_string());
        }

        let four_screen = raw[6] & 0x08 != 0;
        let vertical_mirroring = raw[6] & 0x01 != 0;
        let screen_mirroring = match (four_screen, vertical_mirroring) {
            (true, _) => Mirroring::FourScreen,
            (false, true) => Mirroring::Vertical,
            (false, false) => Mirroring::Horizontal,
        };

        let prg_rom_size = raw[4] as usize * PRG_ROM_PAGE_SIZE;
        let chr_rom_size = raw[5] as usize * CHR_ROM_PAGE_SIZE;

        let skip_trainer = raw[6] & 0x04 != 0;

        let prg_rom_start = 16 + if skip_trainer { 512 } else { 0 };
        let chr_rom_start = prg_rom_start + prg_rom_size;

        println!("PRG ROM size: {}", prg_rom_size);
        println!("CHR ROM size: {}", chr_rom_size);
        println!("Mapper: {}", mapper);
        println!("Mirroring: {:?}", screen_mirroring);
        println!("PRG Rom start: {}", prg_rom_start);
        println!("CHR Rom start: {}", chr_rom_start);

        Ok(Rom {
            prg_rom: raw[prg_rom_start..(prg_rom_start + prg_rom_size)].to_vec(),
            chr_rom: raw[chr_rom_start..(chr_rom_start + chr_rom_size)].to_vec(),
            mapper,
            mirroring: screen_mirroring,
        })
    }
}
