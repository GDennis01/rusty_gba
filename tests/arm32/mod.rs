// pub mod cpu;
use arm7tdmi::arm32::isa::OpcodeArm::{self, *};
use arm7tdmi::cpu::{Opcode, *};
use gba::memory::Memory;

#[cfg(test)]
#[test]
fn mov() {
    let mut cpu: CPU<Memory> = CPU::new();
    cpu.execute_arm(cpu.decode(0xE3A0_0020)); //mov r0,32
    let r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 32); //r0 must be 32
}
#[test]
fn cmp() {
    let mut cpu: CPU<Memory> = CPU::new();
    cpu.execute_arm(cpu.decode(0xE3A0_0020)); //mov r0,32
    let r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 32); //r0 must be 32
    cpu.execute_arm(cpu.decode(0xE350_0020)); //cmp r0,32
    assert!(cpu.psr[cpu.operating_mode].get_z()); //Z must be set to 1
}
