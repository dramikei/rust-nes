mod cpu;
mod bus;
use bus::BUS;
use cpu::CPU;

fn main() {
    println!("NES Started!");
    let mut bus = BUS::new();
    bus.load_cart(String::from("Test"));
    let mut cpu: CPU = CPU::new(bus);
    //////FOR TESTING///////
    cpu.pc = 0xc000;
    loop {
        cpu.bus.system_clock_count += 1;

        //Clock ppu

        if cpu.bus.system_clock_count % 3 == 0 {
            cpu.clock(true);
        }
    }
}

pub fn load_test_rom(bus: &mut BUS) {
    let x = std::include_bytes!("nestest.nes");
    let mut i: usize = 0;
    let prg_size:u32= (x[4] as u32)*16384;
    let chr_size:u32 = (x[5] as u32)*8192;
    println!("PRG ROM SIZE: {} bytes", prg_size);
    println!("CHR ROM SIZE: {} bytes", chr_size);
    if x[6] == 0 {
        while i < prg_size as usize {
            bus.memory[0xc000+i] = x[16+i];
            i += 1;
        }
    }
    
}