use arm7tdmi::cpu::*;
use gba::memory::Memory;
/// Tests provided by https://github.com/jsmolka/gba-tests/blob/master/arm/multiply.asm and
/// decoded,instruction by instruction, through https://shell-storm.org/online/Online-Assembler-and-Disassembler/?inst=cmp+r0%2C0x11&arch=arm&as_format=inline#assembly
#[cfg(test)]
/*Multiply and Multiple Accumultate*/
#[test]
fn multiply1() {
    let mut cpu: CPU<Memory> = CPU::new();
    //mov     r0, 4
    cpu.execute_arm(cpu.decode(0xE3A0_0004));
    let mut r0 = cpu.get_register(0u8);
    assert_eq!(r0, 4);

    // mov     r1, 8
    cpu.execute_arm(cpu.decode(0xE3A0_1008));
    let r1 = cpu.get_register(1u8);
    assert_eq!(r1, 8);

    // mul     r0, r1, r0
    cpu.execute_arm(cpu.decode(0xE000_0091));
    r0 = cpu.get_register(0u8);
    assert_eq!(r0, 32);

    // cmp     r0, 32
    cpu.execute_arm(cpu.decode(0xE350_0020));
    assert!(cpu.psr[cpu.operating_mode].get_z());
}

#[test]
fn multiply2() {
    let mut cpu: CPU<Memory> = CPU::new();
    //mov     r0, -4
    cpu.execute_arm(cpu.decode(0xE3E0_0003));
    let mut r0 = cpu.get_register(0u8) as i32;
    assert_eq!(r0, -4);

    // mov     r1, -8
    cpu.execute_arm(cpu.decode(0xE3E0_1007));
    let r1 = cpu.get_register(1u8) as i32;
    assert_eq!(r1, -8);

    // mul     r0, r1, r0
    cpu.execute_arm(cpu.decode(0xE000_0091));
    r0 = cpu.get_register(0u8) as i32;
    assert_eq!(r0, 32);

    // cmp     r0, 32
    cpu.execute_arm(cpu.decode(0xE350_0020));
    assert!(cpu.psr[cpu.operating_mode].get_z());
}
#[test]
fn multiply3() {
    let mut cpu: CPU<Memory> = CPU::new();
    //mov     r0, 4
    cpu.execute_arm(cpu.decode(0xE3A0_0004));
    let mut r0 = cpu.get_register(0u8) as i32;
    assert_eq!(r0, 4);

    // mov     r1, -8
    cpu.execute_arm(cpu.decode(0xE3E0_1007));
    let r1 = cpu.get_register(1u8) as i32;
    assert_eq!(r1, -8);

    // mul     r0, r1, r0
    cpu.execute_arm(cpu.decode(0xE000_0091));
    r0 = cpu.get_register(0u8) as i32;
    assert_eq!(r0, -32);

    // cmp     r0, -32
    cpu.execute_arm(cpu.decode(0xE370_0020));
    assert!(cpu.psr[cpu.operating_mode].get_z());
}

#[test]
fn multiply_accumulate() {
    let mut cpu: CPU<Memory> = CPU::new();
    //mov     r0, 4
    cpu.execute_arm(cpu.decode(0xE3A0_0004));
    let mut r0 = cpu.get_register(0u8) as i32;
    assert_eq!(r0, 4);

    // mov     r1, 8
    cpu.execute_arm(cpu.decode(0xE3A0_1008));
    let r1 = cpu.get_register(1u8) as i32;
    assert_eq!(r1, 8);

    // mov     r2, 8
    cpu.execute_arm(cpu.decode(0xE3A0_2008));
    let r1 = cpu.get_register(2u8) as i32;
    assert_eq!(r1, 8);

    // mla     r0, r1, r0, r2
    cpu.execute_arm(cpu.decode(0xE020_2091));
    r0 = cpu.get_register(0u8) as i32;
    assert_eq!(r0, 40);

    // cmp     r0, 40
    cpu.execute_arm(cpu.decode(0xE350_0028));
    assert!(cpu.psr[cpu.operating_mode].get_z());
}

#[test]
fn multiply_accumulate2() {
    let mut cpu: CPU<Memory> = CPU::new();
    //mov     r0, 4
    cpu.execute_arm(cpu.decode(0xE3A0_0004));
    let mut r0 = cpu.get_register(0u8) as i32;
    assert_eq!(r0, 4);

    // mov     r1, 8
    cpu.execute_arm(cpu.decode(0xE3A0_1008));
    let r1 = cpu.get_register(1u8) as i32;
    assert_eq!(r1, 8);

    // mov     r2, -8
    cpu.execute_arm(cpu.decode(0xE3E0_2007));
    let r1 = cpu.get_register(2u8) as i32;
    assert_eq!(r1, -8);

    // mla     r0, r1, r0, r2
    cpu.execute_arm(cpu.decode(0xE020_2091));
    r0 = cpu.get_register(0u8) as i32;
    assert_eq!(r0, 24);

    // cmp     r0, 24
    cpu.execute_arm(cpu.decode(0xE350_0018));
    assert!(cpu.psr[cpu.operating_mode].get_z());
}

/*Multiply Long*/
#[test]
fn umull() {
    let mut cpu: CPU<Memory> = CPU::new();
    //mov     r0, 4
    cpu.execute_arm(cpu.decode(0xE3A0_0004));
    let r0 = cpu.get_register(0u8) as i32;
    assert_eq!(r0, 4);

    // mov     r1, 8
    cpu.execute_arm(cpu.decode(0xE3A0_1008));
    let r1 = cpu.get_register(1u8) as i32;
    assert_eq!(r1, 8);

    //        umull   r2, r3, r0, r1
    cpu.execute_arm(cpu.decode(0xE083_2190));

    //        cmp     r2, 32
    cpu.execute_arm(cpu.decode(0xE352_0020));
    assert!(cpu.psr[cpu.operating_mode].get_z());

    //        cmp     r3, 0
    cpu.execute_arm(cpu.decode(0xE353_0000));
    assert!(cpu.psr[cpu.operating_mode].get_z());
}

#[test]
fn umull2() {
    let mut cpu: CPU<Memory> = CPU::new();
    //mov     r0, -1
    cpu.execute_arm(cpu.decode(0xE3E0_0000));
    let r0 = cpu.get_register(0u8) as i32;
    assert_eq!(r0, -1);

    // mov     r1, -1
    cpu.execute_arm(cpu.decode(0xE3E0_1000));
    let r1 = cpu.get_register(1u8) as i32;
    assert_eq!(r1, -1);

    //        umull   r2, r3, r0, r1
    cpu.execute_arm(cpu.decode(0xE083_2190));

    //        cmp     r2, 1
    cpu.execute_arm(cpu.decode(0xE352_0001));
    assert!(cpu.psr[cpu.operating_mode].get_z());

    //        cmp     r3, -2
    cpu.execute_arm(cpu.decode(0xE373_0002));
    assert!(cpu.psr[cpu.operating_mode].get_z());
}
