use core::assert_eq;

use arm7tdmi::cpu::*;
use arm7tdmi::BitRange;
use gba::memory::Memory;
/// Tests provided by https://github.com/jsmolka/gba-tests/blob/master/arm/halfword_transfer.asm and
/// decoded,instruction by instruction, through https://shell-storm.org/online/Online-Assembler-and-Disassembler
///
/// In the tests, "mem" is used as an alias to indicate r11 on which the MEM_IWRAM is moved onto
/// Here, I use r2 instead and I move directly the MEM_IWRAM constant value (in decimal)

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

#[test]
fn load_unsigned_byte() {
    let mut cpu: CPU<Memory> = CPU::new();

    // mov     r2, 50331648 (mem)
    cpu.execute_arm(cpu.decode(0xE3A02403));
    let r2 = cpu.get_register(2u8);
    assert_eq!(r2, 50331648);

    // mov     r0, 0x7F
    cpu.execute_arm(cpu.decode(0xE3A0_007F));
    let r0 = cpu.get_register(0u8);
    assert_eq!(r0, 0x7F);

    // strb    r0, [r2]
    cpu.execute_arm(cpu.decode(0xE5C2_0000));
    // let value = cpu.memory.read_16(r2);
    let value = cpu.memory.read_8(r2);
    assert_eq!(value as u32, 0x7F);

    // ldrsb r1,[r2]
    cpu.execute_arm(cpu.decode(0xE1D2_10D0));
    let r1 = cpu.get_register(1u8);
    assert_eq!(r1, 0x7F);

    // cmp     r1, r0
    cpu.execute_arm(cpu.decode(0xE151_0000));
    assert!(cpu.psr[cpu.operating_mode].get_z());
}

#[test]
fn load_signed_byte() {
    let mut cpu: CPU<Memory> = CPU::new();

    // mov     r2, 50331648 (mem)
    cpu.execute_arm(cpu.decode(0xE3A02403));
    let r2 = cpu.get_register(2u8);
    assert_eq!(r2, 50331648);

    // mov     r0, 0xFF
    cpu.execute_arm(cpu.decode(0xE3A0_00FF));
    let r0 = cpu.get_register(0u8);
    assert_eq!(r0, 0xFF);

    // strb    r0, [r2]
    cpu.execute_arm(cpu.decode(0xE5C2_0000));
    // let value = cpu.memory.read_16(r2);
    let value = cpu.memory.read_8(r2);
    assert_eq!(value as u32, 0xFF);

    // mvn     r0, 0
    cpu.execute_arm(cpu.decode(0xE3E0_0000));
    let r0 = cpu.get_register(0u8);
    assert_eq!(r0, 0xFFFF_FFFF);

    // ldrsb r1,[r2]
    cpu.execute_arm(cpu.decode(0xE1D2_10D0));
    let r1 = cpu.get_register(1u8);
    assert_eq!(r1, 0xFFFF_FFFF);

    // cmp     r1, r0
    cpu.execute_arm(cpu.decode(0xE151_0000));
    assert!(cpu.psr[cpu.operating_mode].get_z());
}

#[test]
fn ldr_str_indexing_wback_offset() {
    let mut cpu: CPU<Memory> = CPU::new();

    // mov     r2, 50331648 (mem)
    cpu.execute_arm(cpu.decode(0xE3A02403));
    let r2 = cpu.get_register(2u8);
    assert_eq!(r2, 50331648);

    // mov     r0, 32
    cpu.execute_arm(cpu.decode(0xE3A0_0020));
    let r0 = cpu.get_register(0u8);
    assert_eq!(r0, 32);

    // mov     r1, 4
    cpu.execute_arm(cpu.decode(0xE3A0_1004));
    let r1 = cpu.get_register(1u8);
    assert_eq!(r1, 4);

    // strh r0, [r2], 4
    cpu.execute_arm(cpu.decode(0xE0C2_00B4));
    let mut r2_updated = cpu.get_register(2u8);
    let value = cpu.memory.read_16(r2);
    assert_eq!(r2 + 4, r2_updated);
    assert_eq!(value, r0 as u16);

    // ldrh r3, [r2,-r1]!
    cpu.execute_arm(cpu.decode(0xE132_30B1));
    r2_updated = cpu.get_register(2u8);
    let r3 = cpu.get_register(3u8);
    assert_eq!(r2_updated, r2);
    assert_eq!(r3, r0);

    // cmp r3, r0
    cpu.execute_arm(cpu.decode(0xE153_0000));
    assert!(cpu.psr[cpu.operating_mode].get_z());

    // cmp r2, 50331648
    cpu.execute_arm(cpu.decode(0xE352_0403));
    assert!(cpu.psr[cpu.operating_mode].get_z());
}

#[test]
fn aligned_store_halfword() {
    let mut cpu: CPU<Memory> = CPU::new();

    // mov     r2, 50331648 (mem)
    cpu.execute_arm(cpu.decode(0xE3A02403));
    let r2 = cpu.get_register(2u8);
    assert_eq!(r2, 50331648);

    // mov     r0, 32
    cpu.execute_arm(cpu.decode(0xE3A0_0020));
    let r0 = cpu.get_register(0u8);
    assert_eq!(r0, 32);

    // strh r0, [r2,1]
    cpu.execute_arm(cpu.decode(0xE1C2_00B1));
    let value = cpu.read_16_aligned_unsigned(r2);
    assert_eq!(value, r0 as u16);

    // ldrh r1, [r2]
    cpu.execute_arm(cpu.decode(0xE1D2_10B0));
    let r1 = cpu.get_register(1u8);
    assert_eq!(r1, r0);

    // cmp r1, r0
    cpu.execute_arm(cpu.decode(0xE151_0000));
    assert!(cpu.psr[cpu.operating_mode].get_z());
}

#[test]
fn misaligned_load_halfword_rotated() {
    let mut cpu: CPU<Memory> = CPU::new();

    // mov     r2, 50331648 (mem)
    cpu.execute_arm(cpu.decode(0xE3A02403));
    let r2 = cpu.get_register(2u8);
    assert_eq!(r2, 50331648);

    // mov     r0, 32
    cpu.execute_arm(cpu.decode(0xE3A0_0020));
    let r0 = cpu.get_register(0u8);
    assert_eq!(r0, 32);

    // strh r0, [r2]
    cpu.execute_arm(cpu.decode(0xE1C2_00B0));
    let value = cpu.memory.read_16(r2);
    assert_eq!(value, r0 as u16);

    // ldrh r1, [r2, 1]
    cpu.execute_arm(cpu.decode(0xE1D2_10B1));
    let r1 = cpu.get_register(1u8);

    // computing  R0 ROR 8
    let overshoot_bits = r0.bit_range(0..8) << (31 - (8 - 1));
    let value = (r0 >> 8) | overshoot_bits;
    assert_eq!(value, r1);

    // cmp r1, r0, ror #8
    cpu.execute_arm(cpu.decode(0xE151_0460));
    assert!(cpu.psr[cpu.operating_mode].get_z());
}

#[test]
fn misaligned_load_signed_halfword() {
    let mut cpu: CPU<Memory> = CPU::new();

    // mov     r2, 50331648 (mem)
    cpu.execute_arm(cpu.decode(0xE3A02403));
    let r2 = cpu.get_register(2u8);
    assert_eq!(r2, 50331648);

    // mov     r0, 0xFF00
    cpu.execute_arm(cpu.decode(0xE3A0_0CFF));
    let r0 = cpu.get_register(0u8);
    assert_eq!(r0, 0xFF00);

    // strh r0, [r2]
    cpu.execute_arm(cpu.decode(0xE1C2_00B0));
    let value = cpu.memory.read_16(r2);
    assert_eq!(value, r0 as u16);

    // mvn     r0, 0
    cpu.execute_arm(cpu.decode(0xE3E0_0000));
    let r0 = cpu.get_register(0u8);
    assert_eq!(r0, 4294967295);

    // ldrsh r1, [r2,1]
    cpu.execute_arm(cpu.decode(0xE1D2_10F1));
    let r1 = cpu.get_register(1u8);
    assert_eq!(r1, 4294967295);

    // cmp r1, r0
    cpu.execute_arm(cpu.decode(0xE151_0000));
    assert!(cpu.psr[cpu.operating_mode].get_z());
}

#[test]
// TODO: idk if this test is successful
fn store_writeback_same_register() {
    let mut cpu: CPU<Memory> = CPU::new();

    // mov      r11, 50331648 (mem)    r11 since r2 will be used later
    cpu.execute_arm(cpu.decode(0xE3A0_B403));
    let r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648);

    // mov      r0, r11
    cpu.execute_arm(cpu.decode(0xE1A0_000B));
    let mut r0 = cpu.get_register(0u8);
    assert_eq!(r11, r0);

    // strh r0, [r0, 4]!
    cpu.execute_arm(cpu.decode(0xE1E0_00B4));
    r0 = cpu.get_register(0u8);
    assert_eq!(r0, 50331648 + 4);

    // add r1, r11, 4
    cpu.execute_arm(cpu.decode(0xE28B_1004));
    let mut r1 = cpu.get_register(1u8);

    // cmp r1, r0
    cpu.execute_arm(cpu.decode(0xE151_0000));
    assert!(cpu.psr[cpu.operating_mode].get_z());
    assert_eq!(r1, r0);

    // ldr     r1, [r0]
    cpu.execute_arm(cpu.decode(0xE590_1000));
    r1 = cpu.get_register(1u8);
    assert_eq!(r1, cpu.memory.read_32(r0));

    // mov      r2, r11
    cpu.execute_arm(cpu.decode(0xE1A0_200B));
    let mut r2 = cpu.get_register(2u8);
    assert_eq!(r11, r2);

    // bic r2, r2, 0xFF00_0000
    cpu.execute_arm(cpu.decode(0xE3C2_24FF));

    // bic     r2, r2, 0xFF0000
    cpu.execute_arm(cpu.decode(0xE3C2_28FF));

    // cmp r2, r1
    cpu.execute_arm(cpu.decode(0xE152_0001));
    assert!(cpu.psr[cpu.operating_mode].get_z());
    r1 = cpu.get_register(1u8);
    r2 = cpu.get_register(2u8);
    assert_eq!(r1, r2);
}

#[test]
// TODO: idk if this test is successful (prolly yes??)
fn store_writeback_same_register2() {
    let mut cpu: CPU<Memory> = CPU::new();

    // mov      r11, 50331648 (mem)    r11 since r2 will be used later
    cpu.execute_arm(cpu.decode(0xE3A0_B403));
    let r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648);

    // mov      r0, r11
    cpu.execute_arm(cpu.decode(0xE1A0_000B));
    let mut r0 = cpu.get_register(0u8);
    assert_eq!(r11, r0);

    // strh r0, [r0, 4]!
    cpu.execute_arm(cpu.decode(0xE1E0_00B4));
    r0 = cpu.get_register(0u8);
    assert_eq!(r0, 50331648 + 4);

    //sub r0,4
    cpu.execute_arm(cpu.decode(0xE240_0004));
    r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 50331648); //r0 must be 32

    // cmp r0, r11
    cpu.execute_arm(cpu.decode(0xE150_000B));
    assert!(cpu.psr[cpu.operating_mode].get_z());
    assert_eq!(r0, r11);

    // ldr     r1, [r0]
    cpu.execute_arm(cpu.decode(0xE590_1000));
    let mut r1 = cpu.get_register(1u8);
    assert_eq!(r1, cpu.memory.read_32(r0));

    // mov      r2, r11
    cpu.execute_arm(cpu.decode(0xE1A0_200B));
    let mut r2 = cpu.get_register(2u8);
    assert_eq!(r11, r2);

    // bic r2, r2, 0xFF00_0000
    cpu.execute_arm(cpu.decode(0xE3C2_24FF));

    // bic     r2, r2, 0xFF0000
    cpu.execute_arm(cpu.decode(0xE3C2_28FF));

    // cmp r2, r1
    cpu.execute_arm(cpu.decode(0xE152_0001));
    assert!(cpu.psr[cpu.operating_mode].get_z());
    r1 = cpu.get_register(1u8);
    r2 = cpu.get_register(2u8);
    assert_eq!(r1, r2);
}

#[test]
fn load_writeback_same_register() {
    let mut cpu: CPU<Memory> = CPU::new();

    // mov      r11, 50331648 (mem)    r11 since r2 will be used later
    cpu.execute_arm(cpu.decode(0xE3A0_B403));
    let r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648);

    // mov      r0, r11
    cpu.execute_arm(cpu.decode(0xE1A0_000B));
    let mut r0 = cpu.get_register(0u8);
    assert_eq!(r11, r0);

    // mov      r1, 32
    cpu.execute_arm(cpu.decode(0xE3A0_1020));
    let r1 = cpu.get_register(1u8);
    assert_eq!(r1, 32);

    // str r1, [r0], #-4
    cpu.execute_arm(cpu.decode(0xE400_1004));
    let value = cpu.memory.read_32(r0);
    assert_eq!(value, 32);

    // ldrh r0, [r0,4]!
    cpu.execute_arm(cpu.decode(0xE1F0_00B4));

    //cmp r0,32
    cpu.execute_arm(cpu.decode(0xE350_0020));
    r0 = cpu.get_register(0u8);
    assert_eq!(r0, 32);
    assert!(cpu.psr[cpu.operating_mode].get_z()); //Z must be set to 1
}
#[test]
fn load_writeback_same_register2() {
    let mut cpu: CPU<Memory> = CPU::new();

    // mov      r11, 50331648 (mem)    r11 since r2 will be used later
    cpu.execute_arm(cpu.decode(0xE3A0_B403));
    let r11 = cpu.get_register(11u8);
    assert_eq!(r11, 50331648);

    // mov      r0, r11
    cpu.execute_arm(cpu.decode(0xE1A0_000B));
    let mut r0 = cpu.get_register(0u8);
    assert_eq!(r11, r0);

    // mov      r1, 32
    cpu.execute_arm(cpu.decode(0xE3A0_1020));
    let r1 = cpu.get_register(1u8);
    assert_eq!(r1, 32);

    // strh r1, [r0]
    cpu.execute_arm(cpu.decode(0xE1C0_10B0));

    // ldrh r0, [r0], 4
    cpu.execute_arm(cpu.decode(0xE0D000B4));

    //cmp r0,32
    cpu.execute_arm(cpu.decode(0xE350_0020));
    r0 = cpu.get_register(0u8);
    assert_eq!(r0, 32);
    assert!(cpu.psr[cpu.operating_mode].get_z()); //Z must be set to 1
}
