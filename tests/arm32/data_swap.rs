use arm7tdmi::cpu::*;
use gba::memory::Memory;

#[cfg(test)]
#[test]
fn swap_word() {
    let mut cpu: CPU<Memory> = CPU::new();
    // mov      r11, 50331648 (mem)    r11 since r2 will be used later
    cpu.execute_arm(cpu.decode(0xE3A0_B403));
    let r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648);

    // mvn r0,0
    cpu.execute_arm(cpu.decode(0xE3E0_0000));
    let r0 = cpu.get_register(0u8);
    assert_eq!(r0, u32::MAX);

    // str r0, [r11]
    cpu.execute_arm(cpu.decode(0xE58B_0000));
    assert_eq!(r0, cpu.memory.read_32(50331648));

    // swp     r1, r0, [r11]
    cpu.execute_arm(cpu.decode(0xE10B_1090));
    let r1 = cpu.get_register(1u8);
    assert_eq!(r1, cpu.memory.read_32(50331648));

    // cmp r1, r0
    cpu.execute_arm(cpu.decode(0xE151_0000));
    let r0 = cpu.get_register(0u8);
    assert!(cpu.psr[cpu.operating_mode].get_z());
    assert_eq!(r1, r0);

    // ldr     r1, [r11]
    cpu.execute_arm(cpu.decode(0xE59B_1000));

    // cmp r1, r0
    cpu.execute_arm(cpu.decode(0xE151_0000));
    let r0 = cpu.get_register(0u8);
    let r1 = cpu.get_register(1u8);
    assert!(cpu.psr[cpu.operating_mode].get_z());
    assert_eq!(r1, r0);
}

#[test]
fn swap_byte() {
    let mut cpu: CPU<Memory> = CPU::new();
    // mov      r11, 50331648 (mem)    r11 since r2 will be used later
    cpu.execute_arm(cpu.decode(0xE3A0_B403));
    let r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648);

    // mvn r0,0
    cpu.execute_arm(cpu.decode(0xE3E0_0000));
    let r0 = cpu.get_register(0u8);
    assert_eq!(r0, u32::MAX);

    // str r0, [r11]
    cpu.execute_arm(cpu.decode(0xE58B_0000));
    assert_eq!(r0, cpu.memory.read_32(50331648));

    // swpb r1,r0, [r11]
    cpu.execute_arm(cpu.decode(0xE14B_1090));
    let r1: u32 = cpu.get_register(1u8);
    assert_eq!(r1, cpu.memory.read_8(50331648) as u32);

    // cmp r1, 0xFF
    cpu.execute_arm(cpu.decode(0xE351_00FF));
    let r1 = cpu.get_register(1u8);
    assert_eq!(r1, 0xFF);

    // ldr     r1, [r11]
    cpu.execute_arm(cpu.decode(0xE59B_1000));

    // cmp r1, r0
    cpu.execute_arm(cpu.decode(0xE151_0000));
    let r0 = cpu.get_register(0u8);
    let r1 = cpu.get_register(1u8);
    assert!(cpu.psr[cpu.operating_mode].get_z());
    assert_eq!(r1, r0);
}
#[test]
fn swap_misaligned() {
    let mut cpu: CPU<Memory> = CPU::new();
    // mov      r11, 50331648 (mem)    r11 since r2 will be used later
    cpu.execute_arm(cpu.decode(0xE3A0_B403));
    let r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648);

    // mov r0, 32
    cpu.execute_arm(cpu.decode(0xE3A0_0020));
    let r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 32);

    //mov r1,64
    cpu.execute_arm(cpu.decode(0xE3A0_1040));
    let r1 = cpu.get_register(1u8);
    assert_eq!(r1, 64);

    // str r1, [r11]
    cpu.execute_arm(cpu.decode(0xE58B_1000));
    assert_eq!(r1, cpu.memory.read_32(50331648));

    // add     r2, r11, 1
    cpu.execute_arm(cpu.decode(0xE28B_2001));
    let r2 = cpu.get_register(2u8);
    let r11 = cpu.get_register(11u8);
    assert_eq!(r2, r11 + 1);

    // swp     r3, r0, [r2]
    cpu.execute_arm(cpu.decode(0xE102_3090));

    // cmp     r3, r1, ror #8
    cpu.execute_arm(cpu.decode(0xE153_0461));
    let r3 = cpu.get_register(3u8);
    let r1 = cpu.get_register(1u8);
    assert_eq!(r1, 64);
    assert_eq!(r3, 0b0100_0000_0000_0000_0000_0000_0000_0000); // 64 ror 8
    assert!(cpu.psr[cpu.operating_mode].get_z());
}
#[test]
fn swap_same_src_dest() {
    let mut cpu: CPU<Memory> = CPU::new();
    // mov      r11, 50331648 (mem)    r11 since r2 will be used later
    cpu.execute_arm(cpu.decode(0xE3A0_B403));
    let r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648);

    // mov r0, 32
    cpu.execute_arm(cpu.decode(0xE3A0_0020));
    let r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 32);

    // str r0, [r11]
    cpu.execute_arm(cpu.decode(0xE58B_0000));
    assert_eq!(r0, cpu.memory.read_32(50331648));

    //mov r0,64
    cpu.execute_arm(cpu.decode(0xE3A0_0040));
    let r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 64); //r0 must be 64

    // swp     r0, r0, [r11]
    cpu.execute_arm(cpu.decode(0xE10B_0090));
    let r0 = cpu.get_register(0u8);
    assert_eq!(r0, 32);
    assert_eq!(cpu.memory.read_32(50331648), 64);

    //cmp r0,32
    cpu.execute_arm(cpu.decode(0xE350_0020));
    assert!(cpu.psr[cpu.operating_mode].get_z());

    // ldr     r0, [r11]
    cpu.execute_arm(cpu.decode(0xE59B_0000));
    let r0 = cpu.get_register(0u8);
    assert_eq!(r0, cpu.memory.read_32(50331648));

    //cmp r0,64
    cpu.execute_arm(cpu.decode(0xE350_0040));
    assert!(cpu.psr[cpu.operating_mode].get_z());
}
