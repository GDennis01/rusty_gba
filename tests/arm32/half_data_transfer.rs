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
    // mov     r2, 50331648 (mem)
    cpu.execute_arm(cpu.decode(0xE3A02403));
    let r2 = cpu.get_register(2u8);
    assert_eq!(r2, 50331648);

    // mvn     r0, 0
    cpu.execute_arm(cpu.decode(0xE3E0_0000));
    let r0 = cpu.get_register(0u8);
    assert_eq!(r0, 4294967295);

    // str     r0, [r2]
    cpu.execute_arm(cpu.decode(0xE582_0000));
    let value = cpu.memory.read_32(r2);
    assert_eq!(value, 4294967295);

    // lsr     r0, 16
    cpu.execute_arm(cpu.decode(0xE1A0_0820));
    let r0 = cpu.get_register(0u8);
    assert_eq!(r0, 64 * 1024 - 1);

    // ldrh    r1, [r2]
    cpu.execute_arm(cpu.decode(0xE1D2_10B0));
    let r1 = cpu.get_register(1u8);
    assert_eq!(r1, 64 * 1024 - 1);

    // cmp     r1, r0
    cpu.execute_arm(cpu.decode(0xE151_0000));
    assert!(cpu.psr[cpu.operating_mode].get_z());
}

#[test]
fn load_unsigned_halfword() {
    let mut cpu: CPU<Memory> = CPU::new();

    // mov     r2, 50331648 (mem)
    cpu.execute_arm(cpu.decode(0xE3A02403));
    let r2 = cpu.get_register(2u8);
    assert_eq!(r2, 50331648);

    // mov     r0, 0x7F00
    cpu.execute_arm(cpu.decode(0xE3A0_0C7F));
    let r0 = cpu.get_register(0u8);
    assert_eq!(r0, 0x7F00);

    // strh    r0, [r2]
    cpu.execute_arm(cpu.decode(0xE1C2_00B0));
    let value = cpu.memory.read_16(r2);
    assert_eq!(value as u32, 0x7F00);

    // ldrsh   r1, [r2]
    cpu.execute_arm(cpu.decode(0xE1D2_10F0));
    let r1 = cpu.get_register(1u8);
    assert_eq!(r1, 0x7F00);

    // cmp     r1, r0
    cpu.execute_arm(cpu.decode(0xE151_0000));
    assert!(cpu.psr[cpu.operating_mode].get_z());
}

#[test]
fn load_signed_halfword() {
    let mut cpu: CPU<Memory> = CPU::new();

    // mov     r2, 50331648 (mem)
    cpu.execute_arm(cpu.decode(0xE3A02403));
    let r2 = cpu.get_register(2u8);
    assert_eq!(r2, 50331648);

    // mov     r0, 0xFF00
    cpu.execute_arm(cpu.decode(0xE3A0_0CFF));
    let r0 = cpu.get_register(0u8);
    assert_eq!(r0, 0xFF00);

    // strh    r0, [r2]
    cpu.execute_arm(cpu.decode(0xE1C2_00B0));
    let value = cpu.memory.read_16(r2);
    assert_eq!(value as u32, 0xFF00);

    // mvn     r0, 0xFF
    cpu.execute_arm(cpu.decode(0xE3E0_00FF));
    let r0 = cpu.get_register(0u8);
    assert_eq!(r0, 0xFFFFFF00);

    // ldrsh   r1, [r2]
    cpu.execute_arm(cpu.decode(0xE1D2_10F0));
    let r1 = cpu.get_register(1u8);
    assert_eq!(r1, 0xFFFF_FF00);

    // cmp     r1, r0
    cpu.execute_arm(cpu.decode(0xE151_0000));
    assert!(cpu.psr[cpu.operating_mode].get_z());
}
