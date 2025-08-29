use arm7tdmi::cpu::*;
use arm7tdmi::BitRange;
use gba::memory::Memory;
/// Tests provided by https://github.com/jsmolka/gba-tests/blob/master/arm/halfword_transfer.asm and
/// decoded,instruction by instruction, through https://shell-storm.org/online/Online-Assembler-and-Disassembler
///
/// In the tests, "mem" is used as an alias to indicate r11 on which the MEM_IWRAM(50331648) is moved onto.
/// Here, I use r2 instead and I move directly the MEM_IWRAM constant value (in decimal).
/// Mnemonics conversion (shellstorm don't support all combinations??)
/// STMFA = STMIB   STMEA = STMIA   STMFD = STMDB   STMED = STMDA
/// LDMFA = LDMDA   LDMEA = LDMDB   LDMFD = LDMIA   LDMED = LDMIB
#[cfg(test)]
#[test]
fn fully_ascending() {
    let mut cpu: CPU<Memory> = CPU::new();
    // mov      r11, 50331648 (mem)    r11 since r2 will be used later
    cpu.execute_arm(cpu.decode(0xE3A0_B403));
    let mut r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648);

    //mov r0,32
    cpu.execute_arm(cpu.decode(0xE3A0_0020));
    let mut r0 = cpu.get_register(0u8);
    assert_eq!(r0, 32);

    //mov r1,64
    cpu.execute_arm(cpu.decode(0xE3A0_1040));
    let mut r1 = cpu.get_register(1u8);
    assert_eq!(r1, 64);

    // stmib r11!, {r0,r1}
    cpu.execute_arm(cpu.decode(0xE9AB_0003));
    r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648 + 8);
    assert_eq!(32, cpu.memory.read_32(50331648 + 4));
    assert_eq!(64, cpu.memory.read_32(50331648 + 8));

    // ldmda   r11!, {r2, r3}
    cpu.execute_arm(cpu.decode(0xE83B_000C));
    r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648);

    //cmp r0,r2
    cpu.execute_arm(cpu.decode(0xE150_0002));
    let r2 = cpu.get_register(2u8);
    r0 = cpu.get_register(0u8);
    assert_eq!(r0, r2);
    assert!(cpu.psr[cpu.operating_mode].get_z());

    //cmp r1,r3
    cpu.execute_arm(cpu.decode(0xE151_0003));
    assert!(cpu.psr[cpu.operating_mode].get_z());
    r1 = cpu.get_register(1u8);
    let r3 = cpu.get_register(3u8);
    assert_eq!(r1, r3);
}

#[test]
fn empty_ascending() {
    let mut cpu: CPU<Memory> = CPU::new();
    // mov      r11, 50331648 (mem)    r11 since r2 will be used later
    cpu.execute_arm(cpu.decode(0xE3A0_B403));
    let mut r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648);

    //mov r0,32
    cpu.execute_arm(cpu.decode(0xE3A0_0020));
    let mut r0 = cpu.get_register(0u8);
    assert_eq!(r0, 32);

    //mov r1,64
    cpu.execute_arm(cpu.decode(0xE3A0_1040));
    let mut r1 = cpu.get_register(1u8);
    assert_eq!(r1, 64);

    // stmia r11!, {r0,r1}
    cpu.execute_arm(cpu.decode(0xE8AB_0003));
    r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648 + 8);
    assert_eq!(32, cpu.memory.read_32(50331648));
    assert_eq!(64, cpu.memory.read_32(50331648 + 4));

    // ldmdb   r11!, {r2, r3}
    cpu.execute_arm(cpu.decode(0xE93B_000C));
    r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648);

    //cmp r0,r2
    cpu.execute_arm(cpu.decode(0xE150_0002));
    let r2 = cpu.get_register(2u8);
    r0 = cpu.get_register(0u8);
    assert_eq!(r0, r2);
    assert!(cpu.psr[cpu.operating_mode].get_z());

    //cmp r1,r3
    cpu.execute_arm(cpu.decode(0xE151_0003));
    assert!(cpu.psr[cpu.operating_mode].get_z());
    r1 = cpu.get_register(1u8);
    let r3 = cpu.get_register(3u8);
    assert_eq!(r1, r3);
}

#[test]
fn fully_descending() {
    let mut cpu: CPU<Memory> = CPU::new();
    // mov      r11, 50331748 (mem)    r11 since r2 will be used later
    cpu.execute_arm(cpu.decode(0xE3A0_B403));
    let mut r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648);

    // add r11, 64
    // to prevent illegal memory access (GBA only)
    cpu.execute_arm(cpu.decode(0xE28B_B040));
    r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648 + 64);

    //mov r0,32
    cpu.execute_arm(cpu.decode(0xE3A0_0020));
    let mut r0 = cpu.get_register(0u8);
    assert_eq!(r0, 32);

    //mov r1,64
    cpu.execute_arm(cpu.decode(0xE3A0_1040));
    let mut r1 = cpu.get_register(1u8);
    assert_eq!(r1, 64);

    // stmdb   r11!, {r0, r1}
    cpu.execute_arm(cpu.decode(0xE92B_0003));
    r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648 + 64 - 8);
    assert_eq!(32, cpu.memory.read_32(50331648 + 64 - 8));
    assert_eq!(64, cpu.memory.read_32(50331648 + 64 - 4));

    // ldmia   r11!, {r2, r3}
    cpu.execute_arm(cpu.decode(0xE8BB_000C));
    r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648 + 64);

    //cmp r0,r2
    cpu.execute_arm(cpu.decode(0xE150_0002));
    let r2 = cpu.get_register(2u8);
    r0 = cpu.get_register(0u8);
    assert_eq!(r0, r2);
    assert!(cpu.psr[cpu.operating_mode].get_z());

    //cmp r1,r3
    cpu.execute_arm(cpu.decode(0xE151_0003));
    assert!(cpu.psr[cpu.operating_mode].get_z());
    r1 = cpu.get_register(1u8);
    let r3 = cpu.get_register(3u8);
    assert_eq!(r1, r3);
}
#[test]
fn empty_descending() {
    let mut cpu: CPU<Memory> = CPU::new();
    // mov      r11, 50331648 (mem)    r11 since r2 will be used later
    cpu.execute_arm(cpu.decode(0xE3A0_B403));
    let mut r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648);

    // add r11, 64
    // to prevent illegal memory access (GBA only)
    cpu.execute_arm(cpu.decode(0xE28B_B040));
    r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648 + 64);

    //mov r0,32
    cpu.execute_arm(cpu.decode(0xE3A0_0020));
    let mut r0 = cpu.get_register(0u8);
    assert_eq!(r0, 32);

    //mov r1,64
    cpu.execute_arm(cpu.decode(0xE3A0_1040));
    let mut r1 = cpu.get_register(1u8);
    assert_eq!(r1, 64);

    // stmda r11!, {r0,r1}
    cpu.execute_arm(cpu.decode(0xE82B_0003));
    r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648 + 64 - 8);
    assert_eq!(32, cpu.memory.read_32(50331648 + 64 - 4));
    assert_eq!(64, cpu.memory.read_32(50331648 + 64));

    // ldmib   r11!, {r2, r3}
    cpu.execute_arm(cpu.decode(0xE9BB_000C));
    r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648 + 64);

    //cmp r0,r2
    cpu.execute_arm(cpu.decode(0xE150_0002));
    let r2 = cpu.get_register(2u8);
    r0 = cpu.get_register(0u8);
    assert_eq!(r0, r2);
    assert!(cpu.psr[cpu.operating_mode].get_z());

    //cmp r1,r3
    cpu.execute_arm(cpu.decode(0xE151_0003));
    assert!(cpu.psr[cpu.operating_mode].get_z());
    r1 = cpu.get_register(1u8);
    let r3 = cpu.get_register(3u8);
    assert_eq!(r1, r3);
}
