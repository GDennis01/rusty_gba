pub use arm7tdmi::cpu::CPU;
use std::{
    fs,
    io::{ErrorKind, Write},
};
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
        .open("log_thumb.txt")
        .unwrap();

    //create an array of 32 bits and store _bios
    let mut bios = [0u32; 0x4000];
    for (i, chunk) in _bios.chunks(4).enumerate() {
        bios[i] = u32::from_le_bytes(chunk.try_into().unwrap());
    }
    //create a new cpu
    let cpu = CPU::new();
    for ins in &bios {
        // Debug for arm
        // let _ins = cpu.decode(*ins);
        // if let Err(e) = writeln!(file, "{_ins}") {
        //     eprintln!("Couldn't write to file: {}", e);
        // }
        // Debug for thumb
        let mut _ins = cpu.decode((*ins & 0xFFFF_0000) >> 16);
        if let Err(e) = writeln!(file, "{_ins}") {
            eprintln!("Couldn't write to file: {}", e);
        }
        _ins = cpu.decode(*ins & 0x0000_FFFF);
        if let Err(e) = writeln!(file, "{_ins}") {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}
