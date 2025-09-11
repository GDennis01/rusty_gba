use core::cell;

use arm7tdmi::cpu::*;
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

#[test]
fn location_fully_ascending() {
    let mut cpu: CPU<Memory> = CPU::new();
    // mov      r11, 50331648 (mem)    r11 since r2 will be used later
    cpu.execute_arm(cpu.decode(0xE3A0_B403));
    let mut r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648);

    //mov r0,32
    cpu.execute_arm(cpu.decode(0xE3A0_0020));
    let mut r0 = cpu.get_register(0u8);
    assert_eq!(r0, 32);

    // stmib   r11, {r0, r1}
    cpu.execute_arm(cpu.decode(0xE98B_0003));
    r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648);
    assert_eq!(32, cpu.memory.read_32(50331648 + 4));
    assert_eq!(0, cpu.memory.read_32(50331648 + 8));

    // ldr     r1, [r11, 4]
    cpu.execute_arm(cpu.decode(0xE59B_1004));

    // cmp     r1, r0
    cpu.execute_arm(cpu.decode(0xE151_0000));
    let r1 = cpu.get_register(1u8);
    r0 = cpu.get_register(0u8);
    assert_eq!(r0, r1);
    assert!(cpu.psr[cpu.operating_mode].get_z());
}

#[test]
fn location_empty_ascending() {
    let mut cpu: CPU<Memory> = CPU::new();
    // mov      r11, 50331648 (mem)    r11 since r2 will be used later
    cpu.execute_arm(cpu.decode(0xE3A0_B403));
    let mut r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648);

    //mov r0,32
    cpu.execute_arm(cpu.decode(0xE3A0_0020));
    let mut r0 = cpu.get_register(0u8);
    assert_eq!(r0, 32);

    // stmia   r11, {r0, r1}
    cpu.execute_arm(cpu.decode(0xE88B_0003));
    r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648);
    assert_eq!(32, cpu.memory.read_32(50331648));
    assert_eq!(0, cpu.memory.read_32(50331648 + 4));

    // ldr     r1, [r11]
    cpu.execute_arm(cpu.decode(0xE59B_1000));

    // cmp     r1, r0
    cpu.execute_arm(cpu.decode(0xE151_0000));
    let r1 = cpu.get_register(1u8);
    r0 = cpu.get_register(0u8);
    assert_eq!(r0, r1);
    assert!(cpu.psr[cpu.operating_mode].get_z());
}

#[test]
fn location_fully_descending() {
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

    // stmdb   r11, {r0, r1}
    cpu.execute_arm(cpu.decode(0xE90B_0003));
    r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648 + 64);
    assert_eq!(32, cpu.memory.read_32(50331648 + 64 - 8));
    assert_eq!(0, cpu.memory.read_32(50331648 + 64 + 4));

    // ldr     r1, [r11,#-8]
    cpu.execute_arm(cpu.decode(0xE51B_1008));

    // cmp     r1, r0
    cpu.execute_arm(cpu.decode(0xE151_0000));
    let r1 = cpu.get_register(1u8);
    r0 = cpu.get_register(0u8);
    assert_eq!(r0, r1);
    assert!(cpu.psr[cpu.operating_mode].get_z());
}

#[test]
fn location_empty_descending() {
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

    // stmda   r11, {r0, r1}
    cpu.execute_arm(cpu.decode(0xE80B_0003));
    r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648 + 64);
    assert_eq!(32, cpu.memory.read_32(50331648 + 64 - 4));
    assert_eq!(0, cpu.memory.read_32(50331648 + 64 + 0));

    // ldr     r1, [r11,#-4]
    cpu.execute_arm(cpu.decode(0xE51B_1004));

    // cmp     r1, r0
    cpu.execute_arm(cpu.decode(0xE151_0000));
    let r1 = cpu.get_register(1u8);
    r0 = cpu.get_register(0u8);
    assert_eq!(r0, r1);
    assert!(cpu.psr[cpu.operating_mode].get_z());
}

#[test]
fn ldm_stm_memory_alignment() {
    // FIXME: It may be broken??
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

    // add r2,r11,3
    cpu.execute_arm(cpu.decode(0xE28B_2003));
    let mut r2 = cpu.get_register(2u8);
    assert_eq!(r2, 50331648 + 64 + 3);

    // sub r3,r11,5
    cpu.execute_arm(cpu.decode(0xE24B_3005));
    let r3 = cpu.get_register(3u8);
    assert_eq!(r3, 50331648 + 64 - 5);

    // stmdb!   r2!, {r0, r1}
    cpu.execute_arm(cpu.decode(0xE922_0003));
    r2 = cpu.get_register(2u8);
    assert_eq!(r2, 50331648 + 64 - 5);
    assert_eq!(32, cpu.memory.read_32(50331648 + 64 - 8));
    assert_eq!(64, cpu.memory.read_32(50331648 + 64 - 4));

    // ldmia   r3, {r4, r5}
    cpu.execute_arm(cpu.decode(0xE893_0030));

    // cmp     r0, r4
    cpu.execute_arm(cpu.decode(0xE150_0004));
    r0 = cpu.get_register(0u8);
    let r4 = cpu.get_register(4u8);
    assert_eq!(r0, r4);
    assert!(cpu.psr[cpu.operating_mode].get_z());

    // cmp     r1, r5
    cpu.execute_arm(cpu.decode(0xE151_0005));
    r1 = cpu.get_register(1u8);
    let r5 = cpu.get_register(5u8);
    assert_eq!(r1, r5);
    assert!(cpu.psr[cpu.operating_mode].get_z());

    // cmp     r2, r3
    cpu.execute_arm(cpu.decode(0xE152_0003));
    r2 = cpu.get_register(2u8);
    let r3 = cpu.get_register(3u8);
    assert_eq!(r2, r3);
    assert!(cpu.psr[cpu.operating_mode].get_z());
}

#[test]
fn ldm_stm_load_pc_store_pc_plus_4() {
    // Load PC test
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

    // adr r1, #4 ( add r1,pc,#4)
    cpu.execute_arm(cpu.decode(0xE28F_1004));
    let r1 = cpu.get_register(1u8);
    let r15 = cpu.get_register(15u8);
    assert_eq!(r1, r15 + 12); // r1 contains the pc at 3 instructions ahead https://github.com/jsmolka/gba-tests/blob/master/arm/block_transfer.asm#L154C10-L154C25

    // stmfd   r11!, {r0, r1}
    cpu.execute_arm(cpu.decode(0xE92B_0003));
    r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648 + 64 - 8);
    assert_eq!(0, cpu.memory.read_32(50331648 + 64 - 8)); // R0 is just 0
    assert_eq!(24, cpu.memory.read_32(50331648 + 64 - 4)); //R1 points at the Store PC+4 test first instruction

    // ldmfd   r11!, {r0, pc}
    cpu.execute_arm(cpu.decode(0xE8BB_8001));
    let r0 = cpu.get_register(0u8);
    let r15 = cpu.get_register(15u8);
    assert_eq!(r0, 0);
    assert_eq!(r15, 24);

    // Store PC+4 test

    // stmfd   r11!, {r0, pc}
    cpu.execute_arm(cpu.decode(0xE92B_8001));
    r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648 + 64 - 8);
    assert_eq!(0, cpu.memory.read_32(50331648 + 64 - 8)); // R0 is just 0
    assert_eq!(36, cpu.memory.read_32(50331648 + 64 - 4)); //PC correctly pointing at next instruction

    // mov     r0, pc
    cpu.execute_arm(cpu.decode(0xE1A0_000F));
    let r0 = cpu.get_register(0u8);
    assert_eq!(r0, 36);

    // ldmfd   r11!, {r1, r2}
    cpu.execute_arm(cpu.decode(0xE8BB_0006));
    let r1 = cpu.get_register(1u8);
    let r2 = cpu.get_register(2u8);
    assert_eq!(r1, 0);
    assert_eq!(r2, 36);

    // cmp     r0, r2
    cpu.execute_arm(cpu.decode(0xE150_0002));
    let r0 = cpu.get_register(0u8);
    let r2 = cpu.get_register(2u8);
    assert_eq!(r0, r2);
    assert!(cpu.psr[cpu.operating_mode].get_z());
}

#[test]
fn ldm_stm_store_user_registers() {
    let mut cpu: CPU<Memory> = CPU::new();
    // changing to priv mode so I can change control bits
    cpu.psr[0].register = 0x1F; //SYS
    cpu.update_operating_mode(false);
    match cpu.operating_mode {
        OperatingMode::System => assert!(true),
        _ => assert!(false),
    }

    // mov      r11, 50331648 (mem)    r11
    cpu.execute_arm(cpu.decode(0xE3A0_B403));
    let mut r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648);

    // add r11, 64
    // to prevent illegal memory access (GBA only)
    cpu.execute_arm(cpu.decode(0xE28B_B040));
    r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648 + 64);

    // mov      r0, r11
    cpu.execute_arm(cpu.decode(0xE1A0_000B));
    let r11 = cpu.get_register(11u8);
    let mut r0 = cpu.get_register(0u8);
    assert_eq!(r0, r11);

    // mov r8, 32
    cpu.execute_arm(cpu.decode(0xE3A0_8020));
    let r8 = cpu.get_register(8u8);
    assert_eq!(r8, 32);

    // msr cpsr, MODE_FIQ
    cpu.execute_arm(cpu.decode(0xE329_F011));
    match cpu.operating_mode {
        OperatingMode::FIQ => assert!(true),
        _ => assert!(false),
    }

    // mov r8, 64
    cpu.execute_arm(cpu.decode(0xE3A0_8040));
    let r8 = cpu.get_register(8u8);
    assert_eq!(r8, 64);

    // stmfd   r0, {r8, r9}^
    cpu.execute_arm(cpu.decode(0xE940_0300));
    r0 = cpu.get_register(0u8);
    assert_eq!(r0, 50331648 + 64);
    assert_eq!(32, cpu.memory.read_32(50331648 + 64 - 8)); // S (^) is set, thus user bank is transferred rather than current mode one

    // sub r0,8
    cpu.execute_arm(cpu.decode(0xE240_0008));
    r0 = cpu.get_register(0u8);
    assert_eq!(r0, 50331648 + 64 - 8);

    // msr cpsr, 0x1F (MODE_SYS)
    cpu.execute_arm(cpu.decode(0xE329_F01F));
    match cpu.operating_mode {
        OperatingMode::System => assert!(true),
        _ => assert!(false),
    }

    // ldmfd   r0, {r1, r2}
    cpu.execute_arm(cpu.decode(0xE890_0006));
    let r1 = cpu.get_register(1u8);
    assert_eq!(r1, 32);

    //cmp r1,32
    cpu.execute_arm(cpu.decode(0xE351_0020));
    assert!(cpu.psr[cpu.operating_mode].get_z()); //Z must be set to 1
}

#[test]
fn ldm_stm_load_user_registers() {
    let mut cpu: CPU<Memory> = CPU::new();
    // changing to priv mode so I can change control bits
    cpu.psr[0].register = 0x1F; //SYS
    cpu.update_operating_mode(false);
    match cpu.operating_mode {
        OperatingMode::System => assert!(true),
        _ => assert!(false),
    }

    // mov      r11, 50331648 (mem)    r11
    cpu.execute_arm(cpu.decode(0xE3A0_B403));
    let mut r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648);

    // add r11, 64
    // to prevent illegal memory access (GBA only)
    cpu.execute_arm(cpu.decode(0xE28B_B040));
    r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648 + 64);

    // mov      r0, r11
    cpu.execute_arm(cpu.decode(0xE1A0_000B));
    let r11 = cpu.get_register(11u8);
    let mut r0 = cpu.get_register(0u8);
    assert_eq!(r0, r11);

    // mov      r1, 0xA
    cpu.execute_arm(cpu.decode(0xE3A0_100A));
    let r1 = cpu.get_register(1u8);
    assert_eq!(r1, 0xA);

    // stmfd   r0!, {r1, r2}
    cpu.execute_arm(cpu.decode(0xE920_0006));
    r0 = cpu.get_register(0u8);
    assert_eq!(r0, 50331648 + 64 - 8);
    assert_eq!(0xA, cpu.memory.read_32(50331648 + 64 - 8));

    // msr cpsr, MODE_FIQ
    cpu.execute_arm(cpu.decode(0xE329_F011));
    match cpu.operating_mode {
        OperatingMode::FIQ => assert!(true),
        _ => assert!(false),
    }

    // mov r8,      0xB
    cpu.execute_arm(cpu.decode(0xE3A0_800B));
    let r8 = cpu.get_register(8u8);
    assert_eq!(r8, 0xB);

    // ldmfd   r0, {r8, r9}^
    cpu.execute_arm(cpu.decode(0xE8D0_0300));
    let r8 = cpu.get_register(8u8);
    assert_eq!(r8, 0xB);

    // msr cpsr, 0x1F (MODE_SYS)
    cpu.execute_arm(cpu.decode(0xE329_F01F));
    match cpu.operating_mode {
        OperatingMode::System => assert!(true),
        _ => assert!(false),
    }

    // cmp r8, 0xA
    cpu.execute_arm(cpu.decode(0xE358_000A));
    let r8 = cpu.get_register(8u8);
    assert_eq!(r8, 0xA);
}

// TODO: this test
#[test]
fn ldm_stm_load_empty_rlist() {
    let mut cpu: CPU<Memory> = CPU::new();

    // mov      r11, 50331648 (mem)    r11
    cpu.execute_arm(cpu.decode(0xE3A0_B403));
    let r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648);

    // adr r0, #8
    cpu.execute_arm(cpu.decode(0xE28F_0008));
    let r0 = cpu.get_register(0u8);
    let r15 = cpu.get_register(15u8);
    assert_eq!(r0, r15 + 16);

    // str     r0, [r11]
    cpu.execute_arm(cpu.decode(0xE58B_0000));
    assert_eq!(r0, cpu.memory.read_32(50331648));

    // mov r0, r11
    cpu.execute_arm(cpu.decode(0xE1A0_000B));
    let r11 = cpu.get_register(11u8);
    let r0 = cpu.get_register(0u8);
    assert_eq!(r0, r11);

    // ldmia r0!, {}
    cpu.execute_arm(cpu.decode(0xE8B00000));
    // rlist is empty, so r15 is transferred instead and r0= r0+0x40
    let r15 = cpu.get_register(15u8);
    assert_eq!(r15, cpu.memory.read_32(r0));

    // sub     r0, 0x40
    cpu.execute_arm(cpu.decode(0xE240_0040));
    let r0 = cpu.get_register(0u8);
    assert_eq!(r0, 50331648); // the ldmia before incremented r0 by 0x40. the sub before brings it back to the original value

    // cmp r0, r11
    cpu.execute_arm(cpu.decode(0xE150_000B));
    let r11 = cpu.get_register(11u8);
    assert_eq!(r0, r11);
    assert!(cpu.psr[cpu.operating_mode].get_z());
}

#[test]
fn stmia_empty_rlist() {
    let mut cpu: CPU<Memory> = CPU::new();

    // mov      r11, 50331648 (mem)    r11
    cpu.execute_arm(cpu.decode(0xE3A0_B403));
    let r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648);

    // mov r0, r11
    cpu.execute_arm(cpu.decode(0xE1A0_000B));
    let r11 = cpu.get_register(11u8);
    let r0 = cpu.get_register(0u8);
    assert_eq!(r0, r11);

    // stmia r0!, {}
    cpu.execute_arm(cpu.decode(0xE8A0_0000));
    let r0 = cpu.get_register(0u8);
    assert_eq!(r0, 50331648 + 0x40);

    // mov r1, pc
    cpu.execute_arm(cpu.decode(0xE1A0_100F));
    let r1 = cpu.get_register(1u8);
    let r15 = cpu.get_register(15u8);
    assert_eq!(r1, r15 + 4); // r1 is pc of this instruction + 8 (see data processing docs)

    // ldr r2, [r11]
    cpu.execute_arm(cpu.decode(0xE59B_2000));

    // cmp r2, r1
    cpu.execute_arm(cpu.decode(0xE152_0001));
    let r2 = cpu.get_register(2u8);
    let r1 = cpu.get_register(1u8);
    assert_eq!(r2, r1);
    assert!(cpu.psr[cpu.operating_mode].get_z());

    // sub     r0, 0x40
    cpu.execute_arm(cpu.decode(0xE240_0040));
    let r0 = cpu.get_register(0u8);
    assert_eq!(r0, 50331648);

    // cmp r0, r11
    cpu.execute_arm(cpu.decode(0xE150_000B));
    let r11 = cpu.get_register(11u8);
    assert_eq!(r0, r11);
    assert!(cpu.psr[cpu.operating_mode].get_z());
}
