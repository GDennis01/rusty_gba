use arm7tdmi::cpu::*;
use gba::memory::Memory;
/// Tests provided by https://github.com/jsmolka/gba-tests/blob/master/arm/single_transfer.asm and
/// decoded,instruction by instruction, through https://shell-storm.org/online/Online-Assembler-and-Disassembler/?inst=cmp+r0%2C0x11&arch=arm&as_format=inline#assembly
#[cfg(test)]
#[test]
fn load_store_word() {
    let mut cpu: CPU<Memory> = CPU::new();
    // mov     r2, 50331648
    cpu.execute_arm(cpu.decode(0xE3A02403));
    let mut r2 = cpu.get_register(2u8);
    assert_eq!(r2, 50331648);

    // mvn     r0, 0
    cpu.execute_arm(cpu.decode(0xE3E0_0000));
    let mut r0 = cpu.get_register(0u8);
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
    let mut r2 = cpu.get_register(2u8);
    assert_eq!(r2, 50331648);

    // mvn     r0, 0
    cpu.execute_arm(cpu.decode(0xE3E0_0000));
    let mut r0 = cpu.get_register(0u8);
    assert_eq!(r0, 4294967295);

    // strb     r0, [r2]
    cpu.execute_arm(cpu.decode(0xE5C2_0000));
    let mut value = cpu.memory.read_32(r2);
    assert_eq!(value, 255);

    // ldr     r1, [r2]
    cpu.execute_arm(cpu.decode(0xE592_1000));
    value = cpu.get_register(1u8);
    assert_eq!(value, cpu.memory.read_32(r2));

    // cmp     r1, 0xFF
    cpu.execute_arm(cpu.decode(0xE351_00FF));
    assert_eq!(cpu.get_register(1u8), 0xFF);
}
#[test]
fn load_byte() {
    let mut cpu: CPU<Memory> = CPU::new();
    // mov     r2, 50331648
    cpu.execute_arm(cpu.decode(0xE3A02403));
    let mut r2 = cpu.get_register(2u8);
    assert_eq!(r2, 50331648);

    // mvn     r0, 0
    cpu.execute_arm(cpu.decode(0xE3E0_0000));
    let mut r0 = cpu.get_register(0u8);
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
    //   mov     r0, 32
    //   mov     r1, 1
    //   mov     r2, mem
    //   str     r0, [r2], 4
    //   ldr     r3, [r2, -r1, lsl 2]!
    //   cmp     r3, r0
    //   bne     f353
    //   cmp     r2, mem
    //   bne     f353
}
