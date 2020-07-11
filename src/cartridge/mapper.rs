pub trait Mapper {
    fn map_prg_read(&self, address: u16) -> u16;
    fn map_prg_write(&mut self, address: u16) -> u16;
    fn map_chr_read(&self, address: u16) -> u16;
    fn map_chr_write(&mut self, address: u16) -> u16;
    fn irq_flag(&self) -> bool {
        false
    }

    fn can_cpu_read(&self, address:u16) -> bool;
    fn can_ppu_read(&self, address:u16) -> bool;
}