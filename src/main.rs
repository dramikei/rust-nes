mod cpu;
mod bus;
use bus::BUS;
use cpu::CPU;

fn main() {
    println!("NES Started!");
    let bus = BUS::new();
    let mut cpu: CPU = CPU::new(bus);
    
}
