use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

#[derive(Debug)]
pub enum Mirroring {
    Horizontal,
    Vertical,
    FourScreen,
}

pub struct NesFile {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub mapper: u8,
    pub mirroring: Mirroring,
}

impl NesFile {
    pub fn load(path: &str) -> io::Result<NesFile> {
        let mut file = File::open(path)?;

        let mut header = [0; 16];
        file.read_exact(&mut header)?;

        if &header[0..4] != b"NES\x1A" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid iNES header",
            ));
        }

        let prg_rom_size = header[4] as usize * 16 * 1024;
        let chr_rom_size = header[5] as usize * 8 * 1024;

        let mirroring = match header[6] & 0x09 {
            0x08 => Mirroring::FourScreen,
            0x01 => Mirroring::Vertical,
            _ => Mirroring::Horizontal,
        };

        let mapper = (header[6] >> 4) | (header[7] & 0xF0);

        file.seek(SeekFrom::Start(16))?;

        let mut prg_rom = vec![0; prg_rom_size];
        file.read_exact(&mut prg_rom)?;

        let mut chr_rom = vec![0; chr_rom_size];
        file.read_exact(&mut chr_rom)?;

        Ok(NesFile {
            prg_rom,
            chr_rom,
            mapper,
            mirroring,
        })
    }
}
