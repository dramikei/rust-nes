use super::Mapper;

pub struct Mapper000 {

}
impl Mapper000 {
    pub fn new() -> Self {
       Mapper000 {

       }
    }
}

impl Mapper for Mapper000 {
    fn read_prg_mapped(&self, address: u16) -> u8 {
        0
    }

    fn write_prg_mapped(&mut self, address: u16, value: u8) {

    }
    fn read_chr_mapped(&self, address: u16) -> u8 {
        0
    }
    fn write_chr_mapped(&mut self, address: u16, value: u8) {

    }
    fn irq_flag(&self) -> bool {
        false
    }
    fn can_read(&self, address: u16) -> bool {
        false
    }
}