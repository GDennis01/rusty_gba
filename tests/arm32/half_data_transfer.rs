use arm7tdmi::cpu::*;
use gba::memory::Memory;
/// Tests provided by https://github.com/jsmolka/gba-tests/blob/master/arm/halfword_transfer.asm and
/// decoded,instruction by instruction, through https://shell-storm.org/online/Online-Assembler-and-Disassembler

#[cfg(test)]
#[test]
fn store_halfword() {
    let mut cpu: CPU<Memory> = CPU::new();
    // mov     r2, 50331648 (mem)
    cpu.execute_arm(cpu.decode(0xE3A02403));
    let r2 = cpu.get_register(2u8);
    assert_eq!(r2, 50331648);

    // mvn     r0, 0
    cpu.execute_arm(cpu.decode(0xE3E0_0000));
    let r0 = cpu.get_register(0u8);
    assert_eq!(r0, 4294967295);

    // strh    r0, [r2]
    cpu.execute_arm(cpu.decode(0xE1C2_00B0));
    let value = cpu.memory.read_16(r2);
    assert_eq!(value as u32, 64 * 1024 - 1);

    // lsr     r0, 16
    cpu.execute_arm(cpu.decode(0xE1A0_0820));
    let r0 = cpu.get_register(0u8);
    assert_eq!(r0, 64 * 1024 - 1);

    // ldr     r1, [r2]
    cpu.execute_arm(cpu.decode(0xE592_1000));
    let r1 = cpu.get_register(1u8);
    assert_eq!(r1, cpu.memory.read_32(r2));

    // cmp     r1, r0
    cpu.execute_arm(cpu.decode(0xE151_0000));
    assert!(cpu.psr[cpu.operating_mode].get_z());
}
#[test]
fn load_halfword() {
    let mut cpu: CPU<Memory> = CPU::new();
    // mvn     r0, 0
    // strh    r0, [r2]
    // lsr     r0, 16
    // ldr     r1, [r2]
    // cmp     r1, r0
    // bne     f400
}
