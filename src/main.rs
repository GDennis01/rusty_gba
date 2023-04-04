pub use arm7tdmi::cpu::MemoryInterface;
pub use arm7tdmi::cpu::CPU;
pub use gba::memory::Memory;
use std::{fs, io::ErrorKind};

pub fn main() {
    println!("Hello, worlds!");
    let _bios = fs::read("gba_bios.bin").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            panic!("Not found");
        } else {
            panic!("IDK");
        }
    });

    //create a file to write in append using fs
    // let mut file = fs::OpenOptions::new()
    //     .write(true)
    //     .append(true)
    //     .open("log_arm.txt")
    //     .unwrap();
    print!("Here");
    //create a new cpu
    let mut cpu: CPU<Memory> = CPU::new();
    cpu.memory.init_bios(_bios);
    cpu.memory.dbg_dump();
}
