mod cpu;
mod bus;
use bus::BUS;
use cpu::CPU;

fn main() {
    println!("NES Started!");
    let mut bus = BUS::new();
    bus.load_cart(String::from("./src/nestest.nes"));
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