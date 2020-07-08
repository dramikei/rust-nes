pub struct CartridgeData {
    pub prg_rom: Vec<u8>,
    pub prg_ram: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub chr_ram: Vec<u8>,
}

impl CartridgeData {
    pub fn new(prg_rom: Vec<u8>, prg_ram: Vec<u8>, chr_rom: Vec<u8>, chr_ram: Vec<u8>) -> Self {
        CartridgeData {
            prg_rom: prg_rom,
            prg_ram: prg_ram,
            chr_rom: chr_rom,
            chr_ram: chr_ram,
        }
    }
}