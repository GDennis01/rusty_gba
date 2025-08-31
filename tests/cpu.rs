use core::cfg;
use std::{fs, io::ErrorKind};

use arm7tdmi::cpu::CPU;
use gba::memory::Memory;

#[cfg(test)]
#[test]
fn test_run_loop() {
    return;
    let mut cpu: CPU<Memory> = CPU::new();
    let _bios = fs::read("gba_bios.bin").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            panic!("GBA Bios Not found");
        } else {
            panic!("IDK");
        }
    });
    cpu.memory.init_bios(_bios);
    cpu.run_loop();
}
