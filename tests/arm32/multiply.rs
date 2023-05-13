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
#[test]
fn umull3() {
    let mut cpu: CPU<Memory> = CPU::new();
    // mov     r0, 2
    cpu.execute_arm(cpu.decode(0xE3A0_0002));
    let r0 = cpu.get_register(0u8) as i32;
    assert_eq!(r0, 2);

    //         mov     r1, -1
    cpu.execute_arm(cpu.decode(0xE3E0_1000));
    let r1 = cpu.get_register(1u8) as i32;
    assert_eq!(r1, -1);

    //         umull   r2, r3, r0, r1
    cpu.execute_arm(cpu.decode(0xE083_2190));

    //         cmp     r2, -2
    cpu.execute_arm(cpu.decode(0xE372_0002));
    assert!(cpu.psr[cpu.operating_mode].get_z());

    //         cmp     r3, 1
    cpu.execute_arm(cpu.decode(0xE353_0001));
    assert!(cpu.psr[cpu.operating_mode].get_z());
}

#[test]
fn umlal() {
    let mut cpu: CPU<Memory> = CPU::new();
    // mov     r0, 4
    cpu.execute_arm(cpu.decode(0xE3A0_0004));
    let r0 = cpu.get_register(0u8);
    assert_eq!(r0, 4);

    // mov     r1, 8
    cpu.execute_arm(cpu.decode(0xE3A0_1008));
    let r1 = cpu.get_register(1u8);
    assert_eq!(r1, 8);

    // mov     r2, 8
    cpu.execute_arm(cpu.decode(0xE3A0_2008));
    let mut r2 = cpu.get_register(2u8);
    assert_eq!(r2, 8);

    // mov     r3, 4
    cpu.execute_arm(cpu.decode(0xE3A0_3004));
    let mut r3 = cpu.get_register(3u8);
    assert_eq!(r3, 4);

    // umlal   r2, r3, r0, r1
    cpu.execute_arm(cpu.decode(0xE0A3_2190));

    //         cmp     r2, 40
    cpu.execute_arm(cpu.decode(0xE352_0028));
    r2 = cpu.get_register(2u8);
    assert_eq!(r2, 40);
    assert!(cpu.psr[cpu.operating_mode].get_z());

    //         cmp     r3, 4
    cpu.execute_arm(cpu.decode(0xE353_0004));
    r3 = cpu.get_register(3u8);
    assert_eq!(r3, 4);
    assert!(cpu.psr[cpu.operating_mode].get_z());
}

#[test]
fn umlal2() {
    let mut cpu: CPU<Memory> = CPU::new();
    // mov     r0, -1
    cpu.execute_arm(cpu.decode(0xE3E0_0000));
    let r0 = cpu.get_register(0u8) as i32;
    assert_eq!(r0, -1);

    // mov     r1, -1
    cpu.execute_arm(cpu.decode(0xE3E0_1000));
    let r1 = cpu.get_register(1u8) as i32;
    assert_eq!(r1, -1);

    // mov     r2, -2
    cpu.execute_arm(cpu.decode(0xE3E0_2001));
    let mut r2 = cpu.get_register(2u8) as i32;
    assert_eq!(r2, -2);

    // mov     r3, 1
    cpu.execute_arm(cpu.decode(0xE3A0_3001));
    let mut r3 = cpu.get_register(3u8) as i32;
    assert_eq!(r3, 1);

    // umlal   r2, r3, r0, r1
    cpu.execute_arm(cpu.decode(0xE0A3_2190));

    //         cmp     r2, -1
    cpu.execute_arm(cpu.decode(0xE372_0001));
    r2 = cpu.get_register(2u8) as i32;
    assert_eq!(r2, -1);
    assert!(cpu.psr[cpu.operating_mode].get_z());

    //         cmp     r3, -1
    cpu.execute_arm(cpu.decode(0xE373_0001));
    r3 = cpu.get_register(3u8) as i32;
    assert_eq!(r3, -1);
    assert!(cpu.psr[cpu.operating_mode].get_z());
}

#[test]
fn smull() {
    let mut cpu: CPU<Memory> = CPU::new();
    // mov     r0, 4
    cpu.execute_arm(cpu.decode(0xE3A0_0004));
    let r0 = cpu.get_register(0u8) as i32;
    assert_eq!(r0, 4);

    // mov     r1, 8
    cpu.execute_arm(cpu.decode(0xE3A0_1008));
    let r1 = cpu.get_register(1u8) as i32;
    assert_eq!(r1, 8);

    // smull   r2, r3, r0, r1
    cpu.execute_arm(cpu.decode(0xE0C3_2190));

    //  cmp     r2, 32
    cpu.execute_arm(cpu.decode(0xE352_0020));
    let r2 = cpu.get_register(2u8) as i32;
    assert_eq!(r2, 32);
    assert!(cpu.psr[cpu.operating_mode].get_z());

    //  cmp     r3, 0
    cpu.execute_arm(cpu.decode(0xE353_0000));
    let r3 = cpu.get_register(3u8) as i32;
    assert_eq!(r3, 0);
    assert!(cpu.psr[cpu.operating_mode].get_z());
}

#[test]
fn smull2() {
    let mut cpu: CPU<Memory> = CPU::new();
    // mov     r0, -4
    cpu.execute_arm(cpu.decode(0xE3E0_0003));
    let r0 = cpu.get_register(0u8) as i32;
    assert_eq!(r0, -4);

    // mov     r1, -8
    cpu.execute_arm(cpu.decode(0xE3E0_1007));
    let r1 = cpu.get_register(1u8) as i32;
    assert_eq!(r1, -8);

    // smull   r2, r3, r0, r1
    cpu.execute_arm(cpu.decode(0xE0C3_2190));

    //  cmp     r2, 32
    cpu.execute_arm(cpu.decode(0xE352_0020));
    let r2 = cpu.get_register(2u8) as i32;
    assert_eq!(r2, 32);
    assert!(cpu.psr[cpu.operating_mode].get_z());

    //  cmp     r3, 0
    cpu.execute_arm(cpu.decode(0xE353_0000));
    let r3 = cpu.get_register(3u8) as i32;
    assert_eq!(r3, 0);
    assert!(cpu.psr[cpu.operating_mode].get_z());
}

#[test]
fn smull3() {
    let mut cpu: CPU<Memory> = CPU::new();
    // mov     r0, 4
    cpu.execute_arm(cpu.decode(0xE3A0_0004));
    let r0 = cpu.get_register(0u8) as i32;
    assert_eq!(r0, 4);

    // mov     r1, -8
    cpu.execute_arm(cpu.decode(0xE3E0_1007));
    let r1 = cpu.get_register(1u8) as i32;
    assert_eq!(r1, -8);

    // smull   r2, r3, r0, r1
    cpu.execute_arm(cpu.decode(0xE0C3_2190));

    //  cmp     r2, -32
    cpu.execute_arm(cpu.decode(0xE372_0020));
    let r2 = cpu.get_register(2u8) as i32;
    assert_eq!(r2, -32);
    assert!(cpu.psr[cpu.operating_mode].get_z());

    //  cmp     r3, -1
    cpu.execute_arm(cpu.decode(0xE373_0001));
    let r3 = cpu.get_register(3u8) as i32;
    assert_eq!(r3, -1);
    assert!(cpu.psr[cpu.operating_mode].get_z());
}

#[test]
fn smlal() {
    let mut cpu: CPU<Memory> = CPU::new();
    // mov     r0, 4
    cpu.execute_arm(cpu.decode(0xE3A0_0004));
    let r0 = cpu.get_register(0u8) as i32;
    assert_eq!(r0, 4);

    // mov     r1, 8
    cpu.execute_arm(cpu.decode(0xE3A0_1008));
    let r1 = cpu.get_register(1u8) as i32;
    assert_eq!(r1, 8);

    // mov     r2, 8
    cpu.execute_arm(cpu.decode(0xE3A0_2008));
    let r2 = cpu.get_register(2u8) as i32;
    assert_eq!(r2, 8);

    // mov     r3, 4
    cpu.execute_arm(cpu.decode(0xE3A0_3004));
    let r3 = cpu.get_register(3u8) as i32;
    assert_eq!(r3, 4);

    // smlal   r2, r3, r0, r1
    cpu.execute_arm(cpu.decode(0xE0E3_2190));

    //  cmp     r2, 40
    cpu.execute_arm(cpu.decode(0xE352_0028));
    let r2 = cpu.get_register(2u8) as i32;
    assert_eq!(r2, 40);
    assert!(cpu.psr[cpu.operating_mode].get_z());

    //  cmp     r3, 4
    cpu.execute_arm(cpu.decode(0xE353_0004));
    let r3 = cpu.get_register(3u8) as i32;
    assert_eq!(r3, 4);
    assert!(cpu.psr[cpu.operating_mode].get_z());
}

#[test]
fn smlal2() {
    let mut cpu: CPU<Memory> = CPU::new();
    // mov     r0, 4
    cpu.execute_arm(cpu.decode(0xE3A0_0004));
    let r0 = cpu.get_register(0u8) as i32;
    assert_eq!(r0, 4);

    // mov     r1, -8
    cpu.execute_arm(cpu.decode(0xE3E0_1007));
    let r1 = cpu.get_register(1u8) as i32;
    assert_eq!(r1, -8);

    // mov     r2, 32
    cpu.execute_arm(cpu.decode(0xE3A0_2020));
    let r2 = cpu.get_register(2u8) as i32;
    assert_eq!(r2, 32);

    // mov     r3, 0
    cpu.execute_arm(cpu.decode(0xE3A0_3000));
    let r3 = cpu.get_register(3u8) as i32;
    assert_eq!(r3, 0);

    // smlal   r2, r3, r0, r1
    cpu.execute_arm(cpu.decode(0xE0E3_2190));

    //  cmp     r2, 0
    cpu.execute_arm(cpu.decode(0xE352_0000));
    let r2 = cpu.get_register(2u8) as i32;
    assert_eq!(r2, 0);
    assert!(cpu.psr[cpu.operating_mode].get_z());

    //  cmp     r3, 0
    cpu.execute_arm(cpu.decode(0xE353_0000));
    let r3 = cpu.get_register(3u8) as i32;
    assert_eq!(r3, 0);
    assert!(cpu.psr[cpu.operating_mode].get_z());
}

// mov     r0, 2
//         mov     r1, 1
//         umulls  r2, r3, r0, r1
//         bmi     f315

//         mov     r0, 2
//         mov     r1, -1
//         smulls  r2, r3, r0, r1
//         bpl     f315
#[test]
fn mul_long_neg_flag() {
    let mut cpu: CPU<Memory> = CPU::new();
    // mov     r0, 2
    cpu.execute_arm(cpu.decode(0xE3A0_0002));
    let r0 = cpu.get_register(0u8) as i32;
    assert_eq!(r0, 2);

    // mov     r1, 1
    cpu.execute_arm(cpu.decode(0xE3A0_1001));
    let r1 = cpu.get_register(1u8) as i32;
    assert_eq!(r1, 1);

    // umulls  r2, r3, r0, r1
    cpu.execute_arm(cpu.decode(0xE093_2190));
    assert!(!cpu.evaluate_cond(Condition::MI));

    // mov     r0, 2
    cpu.execute_arm(cpu.decode(0xE3A0_0002));
    let r0 = cpu.get_register(0u8) as i32;
    assert_eq!(r0, 2);

    // mov     r1, -1
    cpu.execute_arm(cpu.decode(0xE3E0_1000));
    let r1 = cpu.get_register(1u8) as i32;
    assert_eq!(r1, -1);

    // umulls  r2, r3, r0, r1
    cpu.execute_arm(cpu.decode(0xE093_2190));
    assert!(!cpu.evaluate_cond(Condition::PL));
}

#[test]
fn mul_no_c_v_flag() {
    let mut cpu: CPU<Memory> = CPU::new();
    cpu.psr[cpu.operating_mode].register = 0xF000_0000;

    //msr cpsr_f, 0
    cpu.execute_arm(cpu.decode(0xE328_F000));
    assert_eq!(cpu.psr[cpu.operating_mode].register, 0);

    // mov     r0, 1
    cpu.execute_arm(cpu.decode(0xE3A0_0001));
    let r0 = cpu.get_register(0u8) as i32;
    assert_eq!(r0, 1);

    // mov     r1, 1
    cpu.execute_arm(cpu.decode(0xE3A0_1001));
    let r1 = cpu.get_register(0u8) as i32;
    assert_eq!(r1, 1);

    // muls     r0, r1, r0
    cpu.execute_arm(cpu.decode(0xE010_0091));
    assert!(!cpu.evaluate_cond(Condition::CS));
    assert!(!cpu.evaluate_cond(Condition::VS));
}

#[test]
fn mul_no_c_v_flag2() {
    let mut cpu: CPU<Memory> = CPU::new();
    cpu.psr[cpu.operating_mode].register = 0x0000_0000;

    //msr cpsr_f, FLAG C or FLAG V
    cpu.execute_arm(cpu.decode(0xE328_F203));
    assert!(cpu.psr[cpu.operating_mode].get_v());
    assert!(cpu.psr[cpu.operating_mode].get_c());

    // mov     r0, 1
    cpu.execute_arm(cpu.decode(0xE3A0_0001));
    let r0 = cpu.get_register(0u8) as i32;
    assert_eq!(r0, 1);

    // mov     r1, 1
    cpu.execute_arm(cpu.decode(0xE3A0_1001));
    let r1 = cpu.get_register(0u8) as i32;
    assert_eq!(r1, 1);

    // muls     r0, r1, r0
    cpu.execute_arm(cpu.decode(0xE010_0091));
    assert!(!cpu.evaluate_cond(Condition::CC));
    assert!(!cpu.evaluate_cond(Condition::VC));
}

#[test]
fn umull_no_c_v_flag() {
    let mut cpu: CPU<Memory> = CPU::new();
    cpu.psr[cpu.operating_mode].register = 0xF000_0000;

    //msr cpsr_f, 0
    cpu.execute_arm(cpu.decode(0xE328_F000));
    assert_eq!(cpu.psr[cpu.operating_mode].register, 0);

    // mov     r0, 1
    cpu.execute_arm(cpu.decode(0xE3A0_0001));
    let r0 = cpu.get_register(0u8) as i32;
    assert_eq!(r0, 1);

    // mov     r1, 1
    cpu.execute_arm(cpu.decode(0xE3A0_1001));
    let r1 = cpu.get_register(0u8) as i32;
    assert_eq!(r1, 1);

    // umulls   r2, r3, r0, r1
    cpu.execute_arm(cpu.decode(0xE093_2190));
    assert!(!cpu.evaluate_cond(Condition::CS));
    assert!(!cpu.evaluate_cond(Condition::VS));
}

#[test]
fn umull_no_c_v_flag2() {
    let mut cpu: CPU<Memory> = CPU::new();
    cpu.psr[cpu.operating_mode].register = 0x0000_0000;

    //msr cpsr_f, FLAG C or FLAG V
    cpu.execute_arm(cpu.decode(0xE328_F203));
    assert!(cpu.psr[cpu.operating_mode].get_v());
    assert!(cpu.psr[cpu.operating_mode].get_c());

    // mov     r0, 1
    cpu.execute_arm(cpu.decode(0xE3A0_0001));
    let r0 = cpu.get_register(0u8) as i32;
    assert_eq!(r0, 1);

    // mov     r1, 1
    cpu.execute_arm(cpu.decode(0xE3A0_1001));
    let r1 = cpu.get_register(0u8) as i32;
    assert_eq!(r1, 1);

    // umulls   r2, r3, r0, r1
    cpu.execute_arm(cpu.decode(0xE093_2190));
    assert!(!cpu.evaluate_cond(Condition::CC));
    assert!(!cpu.evaluate_cond(Condition::VC));
}

#[test]
fn test() {
    let a: u32 = (-1i32) as u32;
    let b: u32 = (2) as u32;
    println!("a{} b{}", (a as i32 as i64), (b as i64));
    let res: u64 = (a as i64 as u64) * (b as i64 as u64);
    println!("{}", (res as i64));
}
