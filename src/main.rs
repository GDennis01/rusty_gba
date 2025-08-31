pub use arm7tdmi::cpu::MemoryInterface;
pub use arm7tdmi::cpu::CPU;
pub use gba::memory::Memory;
// use std::fmt::Display;
// use std::fmt::Formatter;
use std::io::Write;
use std::iter;
use std::{fs, io::ErrorKind};

pub fn main() {
    let _bios = fs::read("gba_bios.bin").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            panic!("GBA Bios Not found");
        } else {
            panic!("IDK");
        }
    });

    //create a file to write in append using fs
    let mut file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open("dump_txt/log_arm2.txt")
        .unwrap();
    //create a new cpu
    let mut cpu: CPU<Memory> = CPU::new();
    cpu.memory.init_bios(_bios);
    for (i, instr) in cpu.memory.bios.clone().chunks(4).into_iter().enumerate() {
        let instr_as_u32 = u32::from_le_bytes([instr[0], instr[1], instr[2], instr[3]]);
        let instr_fmt = format!("{}:{}\n", i * 4, cpu.decode(instr_as_u32));
        file.write_all(instr_fmt.as_bytes()).unwrap();
    }
    // cpu.memory.dbg_dump();
}
