use arm7tdmi::cpu::*;
use arm7tdmi::BitRange;
use gba::memory::Memory;
/// Tests provided by https://github.com/jsmolka/gba-tests/blob/master/arm/single_transfer.asm and
/// decoded,instruction by instruction, through https://shell-storm.org/online/Online-Assembler-and-Disassembler
///
/// In the tests, "mem" is used as an alias to indicate r11 on which the MEM_IWRAM is moved onto
/// Here, I use r2 instead and I move directly the MEM_IWRAM constant value (in decimal)
#[cfg(test)]
#[test]
fn load_store_word() {
    let mut cpu: CPU<Memory> = CPU::new();
    // mov     r2, 50331648
    cpu.execute_arm(cpu.decode(0xE3A02403));
    let r2 = cpu.get_register(2u8);
    assert_eq!(r2, 50331648);

    // mvn     r0, 0
    cpu.execute_arm(cpu.decode(0xE3E0_0000));
    let r0 = cpu.get_register(0u8);
    assert_eq!(r0, 4294967295);

    // str     r0, [r2]
    cpu.execute_arm(cpu.decode(0xE582_0000));
    let mut value = cpu.memory.read_32(r2);
    assert_eq!(value, r0);

    // ldr     r1, [r2]
    cpu.execute_arm(cpu.decode(0xE592_1000));
    value = cpu.get_register(1u8);
    assert_eq!(value, cpu.memory.read_32(r2));

    // cmp     r1, r0
    cpu.execute_arm(cpu.decode(0xE151_0000));
    assert_eq!(cpu.get_register(1u8), cpu.get_register(0u8));
}
#[test]
fn store_byte() {
    let mut cpu: CPU<Memory> = CPU::new();
    // mov     r2, 50331648
    cpu.execute_arm(cpu.decode(0xE3A02403));
    let r2 = cpu.get_register(2u8);
    assert_eq!(r2, 50331648);

    // mvn     r0, 0
    cpu.execute_arm(cpu.decode(0xE3E0_0000));
    let r0 = cpu.get_register(0u8);
    assert_eq!(r0, 4294967295);

    // strb     r0, [r2]
    cpu.execute_arm(cpu.decode(0xE5C2_0000));
    let value = cpu.memory.read_8(r2);
    assert_eq!(value, 255);

    // ldr     r1, [r2]
    cpu.execute_arm(cpu.decode(0xE592_1000));
    let value2 = cpu.get_register(1u8);
    assert_eq!(value2, cpu.memory.read_32(r2));

    // cmp     r1, 0xFF
    cpu.execute_arm(cpu.decode(0xE351_00FF));
    assert_eq!(cpu.get_register(1u8), 0xFF);
}
#[test]
fn load_byte() {
    let mut cpu: CPU<Memory> = CPU::new();
    // mov     r2, 50331648
    cpu.execute_arm(cpu.decode(0xE3A02403));
    let r2 = cpu.get_register(2u8);
    assert_eq!(r2, 50331648);

    // mvn     r0, 0
    cpu.execute_arm(cpu.decode(0xE3E0_0000));
    let r0 = cpu.get_register(0u8);
    assert_eq!(r0, 4294967295);

    // str     r0, [r2]
    cpu.execute_arm(cpu.decode(0xE582_0000));
    let mut value = cpu.memory.read_32(r2);
    assert_eq!(value, r0);

    // ldrb    r1, [r2]
    cpu.execute_arm(cpu.decode(0xE5D2_1000));
    value = cpu.get_register(1u8);
    assert_eq!(value, cpu.memory.read_8(r2) as u32);

    // cmp     r1, 0xFF
    cpu.execute_arm(cpu.decode(0xE351_00FF));
    assert_eq!(cpu.get_register(1u8), 0xFF);
}
#[test]
fn index_writeback() {
    let mut cpu: CPU<Memory> = CPU::new();
    //   mov     r0, 32
    cpu.execute_arm(cpu.decode(0xE3A0_0020));
    let mut r0 = cpu.get_register(0u8);
    assert_eq!(r0, 32);

    //   mov     r1, 1
    cpu.execute_arm(cpu.decode(0xE3A0_1001));
    let r1 = cpu.get_register(1u8);
    assert_eq!(r1, 1);

    //   mov     r2, mem = 0x03000000
    cpu.execute_arm(cpu.decode(0xE3A0_2403));
    let r2 = cpu.get_register(2u8);
    assert_eq!(r2, 0x0300_0000);

    //   str     r0, [r2], 4
    cpu.execute_arm(cpu.decode(0xE482_0004));
    let read_value = cpu.memory.read_32(r2);

    assert_eq!(read_value, 32);

    //   ldr     r3, [r2, -r1, lsl 2]!
    cpu.execute_arm(cpu.decode(0xE7323101));
    let r3 = cpu.get_register(3u8);
    assert_eq!(r3, 32);

    //   cmp     r3, r0
    cpu.execute_arm(cpu.decode(0xE153_0000));
    assert!(cpu.evaluate_cond(Condition::EQ));

    //   bne     f353
    //   cmp     r2, mem
    cpu.execute_arm(cpu.decode(0xE352_0403));
    assert!(cpu.evaluate_cond(Condition::EQ));
    assert_eq!(cpu.get_register(2u8), 0x03000000)
    //   bne     f353
}

#[test]
fn misaligned_store() {
    let mut cpu: CPU<Memory> = CPU::new();

    // mov     r0, 32
    cpu.execute_arm(cpu.decode(0xE3A0_0020));
    let r0 = cpu.get_register(0u8);
    assert_eq!(r0, 32);

    //   mov     r2, mem = 0x03000000
    cpu.execute_arm(cpu.decode(0xE3A0_2403));
    let r2 = cpu.get_register(2u8);
    assert_eq!(r2, 0x0300_0000);

    // str     r0, [r2, 3]
    cpu.execute_arm(cpu.decode(0xE582_0003));
    let read_value = cpu.memory.read_32(r2);
    assert_eq!(read_value, 32);

    // ldr     r1, [r2]
    cpu.execute_arm(cpu.decode(0xE592_1000));
    assert_eq!(cpu.get_register(1u8), cpu.get_register(0u8));

    // cmp     r1, r0
    cpu.execute_arm(cpu.decode(0xE151_0000));
    assert_eq!(cpu.get_register(1u8), cpu.get_register(0u8));
}
#[test]
fn misaligned_load() {
    let mut cpu: CPU<Memory> = CPU::new();
    // mov     r0, 32
    cpu.execute_arm(cpu.decode(0xE3A0_0020));
    let r0 = cpu.get_register(0u8);
    assert_eq!(r0, 32);

    //   mov     r2, mem = 0x03000000
    cpu.execute_arm(cpu.decode(0xE3A0_2403));
    let r2 = cpu.get_register(2u8);
    assert_eq!(r2, 0x0300_0000);

    // str     r0, [r2]
    cpu.execute_arm(cpu.decode(0xE582_0000));
    let value = cpu.memory.read_32(r2);
    assert_eq!(value, r0);

    // ldr     r1, [r2, 3]
    cpu.execute_arm(cpu.decode(0xE592_1003));
    let r1 = cpu.get_register(1u8);
    // computing  R0 ROR 24
    let overshoot_bits = r0.bit_range(0..24) << (31 - (24 - 1));
    let value = (r0 >> 24) | overshoot_bits;
    assert_eq!(value, r1);
    // cmp     r1, r0, ror 24
    cpu.execute_arm(cpu.decode(0xE151_0C60));
    assert!(cpu.psr[cpu.operating_mode].get_c());
}
#[test]
fn store_pc() {
    let mut cpu: CPU<Memory> = CPU::new();

    //   mov     r2, mem = 0x03000000
    cpu.execute_arm(cpu.decode(0xE3A0_2403));
    let r2 = cpu.get_register(2u8);
    assert_eq!(r2, 0x0300_0000);

    //  str     pc, [r2]
    cpu.execute_arm(cpu.decode(0xE582_F000));
    let r15 = cpu.get_register(15u8);
    let value = cpu.memory.read_32(r2);
    assert_eq!(value, r15 + 8);

    // mov     r0, pc
    cpu.execute_arm(cpu.decode(0xE1A0_000F));
    let r0 = cpu.get_register(0u8);

    // ldr     r1, [r2]
    cpu.execute_arm(cpu.decode(0xE592_1000));
    let r1 = cpu.get_register(1u8);
    assert_eq!(r1, r0);

    // cmp     r1, r0
    cpu.execute_arm(cpu.decode(0xE151_0000));
    assert!(cpu.evaluate_cond(Condition::EQ));
}

#[test]
#[should_panic]
fn load_rrx_as_offset() {
    let mut cpu: CPU<Memory> = CPU::new();
    cpu.execute_arm(cpu.decode(0xE3A0_0000));
    let r0 = cpu.get_register(0u8);
    assert_eq!(r0, 0);
    cpu.execute_arm(cpu.decode(0xE3A0_1000));
    let r1 = cpu.get_register(1u8);
    assert_eq!(r1, 0);
    // msr     cpsr_f, FLAG_C(1<<29)
    cpu.execute_arm(cpu.decode(0xE328_F202));
    assert!(cpu.psr[cpu.operating_mode].get_c());
    assert!(!cpu.psr[cpu.operating_mode].get_z());
    assert!(!cpu.psr[cpu.operating_mode].get_v());
    assert!(!cpu.psr[cpu.operating_mode].get_n());
    // ldr     r2, [r1, r0, rrx]!
    //it should panic as the address generated by RRX is out of the memory map of GBA
    cpu.execute_arm(cpu.decode(0xE7B1_2060));

    // cmp     r1, 1 shl 31
    cpu.execute_arm(cpu.decode(0xE351_0102));
    assert!(cpu.evaluate_cond(Condition::EQ));
}
