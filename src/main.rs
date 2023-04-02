pub use arm7tdmi::cpu::MemoryInterface;
pub use arm7tdmi::cpu::CPU;
use std::{
    fs,
    io::{ErrorKind, Write},
};

//Debug Implementation of memory
pub struct Memory {
    //memory is byte addressable, not word addressable
    memory: [u8; 4000],
}
impl Default for Memory {
    fn default() -> Self {
        Memory { memory: [0; 4000] }
    }
}
impl MemoryInterface for Memory {
    fn new() -> Self {
        Memory { memory: [0; 4000] }
    }
    fn read_8(&self, address: u32) -> u8 {
        self.memory[address as usize]
    }
    fn read_16(&self, address: u32) -> u16 {
        1
    }
    fn read_32(&self, address: u32) -> u32 {
        1
    }
    fn write_8(&mut self, address: u32, data: u8) {}
    fn write_16(&mut self, address: u32, data: u16) {}
    fn write_32(&mut self, address: u32, data: u32) {}
}
//import lib
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
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("log_arm.txt")
        .unwrap();

    //create an array of 32 bits and store _bios
    let mut bios = [0u32; 0x4000];
    for (i, chunk) in _bios.chunks(4).enumerate() {
        bios[i] = u32::from_le_bytes(chunk.try_into().unwrap());
    }
    //create a new cpu
    let cpu: CPU<Memory> = CPU::new();
    for ins in &bios {
        // Debug for arm
        let _ins = cpu.decode(*ins);
        if let Err(e) = writeln!(file, "{_ins}") {
            eprintln!("Couldn't write to file: {}", e);
        }
        // Debug for thumb
        // let mut _ins = cpu.decode((*ins & 0xFFFF_0000) >> 16);
        // if let Err(e) = writeln!(file, "{_ins}") {
        //     eprintln!("Couldn't write to file: {}", e);
        // }
        // _ins = cpu.decode(*ins & 0x0000_FFFF);
        // if let Err(e) = writeln!(file, "{_ins}") {
        //     eprintln!("Couldn't write to file: {}", e);
        // }
    }
}
