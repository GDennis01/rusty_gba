use arm7tdmi::cpu::*;
use gba::memory::Memory;
/// Tests provided by https://github.com/jsmolka/gba-tests/blob/master/arm/data_processing.asm and
/// decoded,instruction by instruction, through https://shell-storm.org/online/Online-Assembler-and-Disassembler/?inst=cmp+r0%2C0x11&arch=arm&as_format=inline#assembly
#[cfg(test)]
#[test]
fn dp_mov() {
    let mut cpu: CPU<Memory> = CPU::new();
    //mov r0,32
    cpu.execute_arm(cpu.decode(0xE3A0_0020));
    let r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 32); //r0 must be 32
}
#[test]
fn dp_mvn() {
    let mut cpu: CPU<Memory> = CPU::new();
    //mvn r0,0 -> not 0 -> all 1's
    cpu.execute_arm(cpu.decode(0xE3E0_0000));
    let mut r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 0xFFFF_FFFF); //r0 must all 1's

    //adds r0,1  -> all 1's + 1 = 0
    cpu.execute_arm(cpu.decode(0xE290_0001));
    r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 0); //r0 must be all 0

    assert!(cpu.psr[cpu.operating_mode].get_z()); //Z must be set to 1
}

#[test]
fn dp_and() {
    let mut cpu: CPU<Memory> = CPU::new();
    //mov r0,0xFF
    cpu.execute_arm(cpu.decode(0xE3A0_00FF));
    let mut r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 0xFF); //r0 must be 0xFF

    //and r0,0x0F
    cpu.execute_arm(cpu.decode(0xE200_000F));
    r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 0x0F); //r0 must be 0x0F

    //cmp r0,0x0F
    cpu.execute_arm(cpu.decode(0xE350_000F));
    assert!(cpu.psr[cpu.operating_mode].get_z()); //Z must be set to 1
}

#[test]
fn dp_eor() {
    let mut cpu: CPU<Memory> = CPU::new();
    //mov r0,0xFF
    cpu.execute_arm(cpu.decode(0xE3A0_00FF));
    let mut r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 0xFF); //r0 must be 0xFF

    //eor r0,0xF0
    cpu.execute_arm(cpu.decode(0xE220_00F0));
    r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 0xF); //r0 must be 0x0F

    //cmp r0,0x0F
    cpu.execute_arm(cpu.decode(0xE350_000F));
    assert!(cpu.psr[cpu.operating_mode].get_z()); //Z must be set to 1
}

#[test]
fn dp_or() {
    let mut cpu: CPU<Memory> = CPU::new();
    //mov r0,0xF0
    cpu.execute_arm(cpu.decode(0xE3A0_00F0));
    let mut r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 0xF0); //r0 must be 0xFF

    //orr r0,0x0F
    cpu.execute_arm(cpu.decode(0xE380_000F));
    r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 0xFF); //r0 must be 0xFF

    //cmp r0,0xFF
    cpu.execute_arm(cpu.decode(0xE350_00FF));
    assert!(cpu.psr[cpu.operating_mode].get_z()); //Z must be set to 1
}

#[test]
fn dp_bic() {
    let mut cpu: CPU<Memory> = CPU::new();
    //mov r0,0xFF
    cpu.execute_arm(cpu.decode(0xE3A0_00FF));
    let mut r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 0xFF); //r0 must be 0xFF

    //bic r0,0x0F
    cpu.execute_arm(cpu.decode(0xE3C0_000F));
    r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 0xF0); //r0 must be 0xF0

    //cmp r0,0xF0
    cpu.execute_arm(cpu.decode(0xE350_00F0));
    assert!(cpu.psr[cpu.operating_mode].get_z()); //Z must be set to 1
}

#[test]
fn dp_add() {
    let mut cpu: CPU<Memory> = CPU::new();
    //mov r0,32
    cpu.execute_arm(cpu.decode(0xE3A0_0020));
    let mut r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 32); //r0 must be 32

    //add r0,32
    cpu.execute_arm(cpu.decode(0xE280_0020));
    r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 64); //r0 must be 32

    //cmp r0,32
    cpu.execute_arm(cpu.decode(0xE350_0040));
    assert!(cpu.psr[cpu.operating_mode].get_z()); //Z must be set to 1
}
#[test]
fn dp_adc() {
    let mut cpu: CPU<Memory> = CPU::new();
    //msr cpsr_f,0  (cpsr_f->sets only flag bits) aka resets flags bits
    cpu.execute_arm(cpu.decode(0xE328_F000));
    assert_eq!(cpu.psr[0].register, 0);

    //movs r0,32
    cpu.execute_arm(cpu.decode(0xE3B0_0020));
    let mut r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 32);

    //adc r0,32
    cpu.execute_arm(cpu.decode(0xE2A0_0020));
    r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 64);

    //cmp r0,64
    cpu.execute_arm(cpu.decode(0xE350_0040));
    assert!(cpu.psr[cpu.operating_mode].get_z());

    //msr  cpsr, FLAG_C
    cpu.execute_arm(cpu.decode(0xE328_F202));
    assert!(cpu.psr[cpu.operating_mode].get_c());

    //mov r0,32
    cpu.execute_arm(cpu.decode(0xE3A0_0020));
    r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 32);

    // adc r0, 32                     E2A0_0020
    cpu.execute_arm(cpu.decode(0xE2A0_0020));
    r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 65);

    // cmp r0, 65                     E350_0041
    cpu.execute_arm(cpu.decode(0xE350_0041));
    assert!(cpu.psr[cpu.operating_mode].get_c() && cpu.psr[cpu.operating_mode].get_z());
}

#[test]
fn dp_sub() {
    let mut cpu: CPU<Memory> = CPU::new();
    //mov r0,64
    cpu.execute_arm(cpu.decode(0xE3A0_0040));
    let mut r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 64); //r0 must be 64

    //sub r0,32
    cpu.execute_arm(cpu.decode(0xE240_0020));
    r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 32); //r0 must be 32

    //cmp r0,32
    cpu.execute_arm(cpu.decode(0xE350_0020));
    assert!(cpu.psr[cpu.operating_mode].get_z()); //Z must be set to 1
}

#[test]
fn dp_rsb() {
    let mut cpu: CPU<Memory> = CPU::new();
    //mov r0,32
    cpu.execute_arm(cpu.decode(0xE3A0_0020));
    let mut r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 32); //r0 must be 32

    //sub r0,64
    cpu.execute_arm(cpu.decode(0xE260_0040));
    r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 32); //r0 must be 32

    //cmp r0,32
    cpu.execute_arm(cpu.decode(0xE350_0020));
    assert!(cpu.psr[cpu.operating_mode].get_z());
    //Z must be set to 1
}

#[test]
fn dp_sbc() {
    let mut cpu: CPU<Memory> = CPU::new();
    //msr cpsr_f,0  (cpsr_f->sets only flag bits) aka resets flags bits
    cpu.execute_arm(cpu.decode(0xE328_F000));
    assert_eq!(cpu.psr[0].register, 0);

    //mov r0,64
    cpu.execute_arm(cpu.decode(0xE3A0_0040));
    let mut r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 64);

    //sbc r0,32
    cpu.execute_arm(cpu.decode(0xE2C0_0020));
    r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 31);

    //cmp r0,31
    cpu.execute_arm(cpu.decode(0xE350_001F));
    assert!(cpu.psr[cpu.operating_mode].get_z());

    //msr  cpsr, FLAG_C
    cpu.execute_arm(cpu.decode(0xE328_F202));
    assert!(cpu.psr[cpu.operating_mode].get_c());

    //mov r0,64
    cpu.execute_arm(cpu.decode(0xE3A0_0040));
    r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 64);

    // sbc r0, 32
    cpu.execute_arm(cpu.decode(0xE2C0_0020));
    r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 32);

    // cmp r0, 32
    cpu.execute_arm(cpu.decode(0xE350_0020));
    assert!(cpu.psr[cpu.operating_mode].get_z() && cpu.psr[cpu.operating_mode].get_c());
}

#[test]
fn dp_rsc() {
    let mut cpu: CPU<Memory> = CPU::new();
    //msr cpsr_f,0  (cpsr_f->sets only flag bits) aka resets flags bits
    cpu.execute_arm(cpu.decode(0xE328_F000));
    assert_eq!(cpu.psr[0].register, 0);

    //mov r0,32
    cpu.execute_arm(cpu.decode(0xE3A0_0020));
    let mut r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 32);

    //rsc     r0, 64
    cpu.execute_arm(cpu.decode(0xE2E0_0040));
    r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 31);

    //cmp r0,31
    cpu.execute_arm(cpu.decode(0xE350_001F));
    assert!(cpu.psr[cpu.operating_mode].get_z());

    //msr  cpsr, FLAG_C
    cpu.execute_arm(cpu.decode(0xE328_F202));
    assert!(cpu.psr[cpu.operating_mode].get_c());

    //mov r0,32
    cpu.execute_arm(cpu.decode(0xE3A0_0020));
    r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 32);

    //rsc  r0, 64
    cpu.execute_arm(cpu.decode(0xE2E0_0040));
    r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 32);

    // cmp r0, 32
    cpu.execute_arm(cpu.decode(0xE350_0020));
    assert!(cpu.psr[cpu.operating_mode].get_z() && cpu.psr[cpu.operating_mode].get_c());
}

#[test]
fn dp_cmp() {
    let mut cpu: CPU<Memory> = CPU::new();
    //mov r0,32
    cpu.execute_arm(cpu.decode(0xE3A0_0020));
    let r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 32); //r0 must be 32

    //cmp r0,32
    cpu.execute_arm(cpu.decode(0xE350_0020));
    assert!(cpu.psr[cpu.operating_mode].get_z()); //Z must be set to 1
}

#[test]
fn dp_cmn() {
    let mut cpu: CPU<Memory> = CPU::new();
    //mov r0, 1 shl 31 -> 1 << 31 -> 0x8000_0000
    cpu.execute_arm(cpu.decode(0xE3A0_0102));
    let r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 0x8000_0000); //r0 must be 0x8000_0000

    //cmn r0,r0 -> 0x8000_0000+0x8000_0000 would result in 1_0000_0000 which overflows so results would be bits [0..=31]
    cpu.execute_arm(cpu.decode(0xE170_0000));

    assert!(cpu.psr[cpu.operating_mode].get_z()); //Z must be set to 1
    assert!(cpu.psr[cpu.operating_mode].get_v()); //V must be set to 1
}

#[test]
fn dp_tst() {
    let mut cpu: CPU<Memory> = CPU::new();
    // mov r0, 0xF0
    cpu.execute_arm(cpu.decode(0xE3A0_00F0));
    let r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 0xF0); //r0 must be 0xF0

    //tst r0,0x0F
    cpu.execute_arm(cpu.decode(0xE310_000F));

    assert!(cpu.psr[cpu.operating_mode].get_z()); //Z must be set to 1
}

#[test]
fn dp_teq() {
    let mut cpu: CPU<Memory> = CPU::new();
    // mov r0, 0xFF
    cpu.execute_arm(cpu.decode(0xE3A0_00FF));
    let r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 0xFF); //r0 must be 0xFF

    //teq r0,0xFF
    cpu.execute_arm(cpu.decode(0xE330_00FF));

    assert!(cpu.psr[cpu.operating_mode].get_z()); //Z must be set to 1
}
#[test]
fn dp_lsl() {
    let mut cpu: CPU<Memory> = CPU::new();
    // mov     r0, 0xFF00
    cpu.execute_arm(cpu.decode(0xE3A0_0CFF));
    let r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 0xFF00);

    //mov r1,0x00FF
    cpu.execute_arm(cpu.decode(0xE3A0_10FF));
    let r1 = cpu.get_register(1 as u8);
    assert_eq!(r1, 0xFF);

    //mov r1, r1, lsl 8
    cpu.execute_arm(cpu.decode(0xE1A0_1401));
    let r1 = cpu.get_register(1 as u8);
    assert_eq!(r1, 0xFF00);

    cpu.execute_arm(cpu.decode(0xE151_0000));
    assert!(cpu.psr[cpu.operating_mode].get_z());
}

#[test]
fn dp_update_carry_rotate_immediate() {
    let mut cpu: CPU<Memory> = CPU::new();
    // movs     r0, 0xF000000F
    cpu.execute_arm(cpu.decode(0xE3B0_02FF));
    let mut r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 0xF000_000F);
    assert!(!cpu.evaluate_cond(Condition::CC));

    //movs    r0, 0x0FF00000
    cpu.execute_arm(cpu.decode(0xE3B0_06FF));
    r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 0x0FF00000);
    assert!(!cpu.evaluate_cond(Condition::CS));
}

#[test]
fn dp_update_carry_rotate_register() {
    let mut cpu: CPU<Memory> = CPU::new();
    //mov     r0, 0xFF
    cpu.execute_arm(cpu.decode(0xE3A0_00FF));
    let mut r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 0xFF);

    // mov     r1, 4
    cpu.execute_arm(cpu.decode(0xE3A0_1004));
    let mut r1 = cpu.get_register(1 as u8);
    assert_eq!(r1, 4);

    //movs r2,r0,ror r1
    cpu.execute_arm(cpu.decode(0xE1B0_2170));
    let r2 = cpu.get_register(2 as u8);
    assert_eq!(r2, 0xF000_000F);
    assert!(!cpu.evaluate_cond(Condition::CC));

    // mov     r0, 0xF0
    cpu.execute_arm(cpu.decode(0xE3A0_00F0));
    r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 0xF0);

    // mov     r1, 4
    cpu.execute_arm(cpu.decode(0xE3A0_1004));
    r1 = cpu.get_register(1 as u8);
    assert_eq!(r1, 4);

    //movs r2,r0,ror r1
    cpu.execute_arm(cpu.decode(0xE1B0_2170));
    let r2 = cpu.get_register(2 as u8);
    assert_eq!(r2, 0xF);
    assert!(!cpu.evaluate_cond(Condition::CS));
}

#[test]
fn dp_update_carry_rotate_register2() {
    let mut cpu: CPU<Memory> = CPU::new();
    //mov     r0, 0xFF
    cpu.execute_arm(cpu.decode(0xE3A0_00FF));
    let mut r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 0xFF);

    //movs r1,r0,ror 4
    cpu.execute_arm(cpu.decode(0xE1B0_1260));
    let mut r1 = cpu.get_register(1 as u8);
    assert_eq!(r1, 0xF000_000F);
    assert!(!cpu.evaluate_cond(Condition::CC));

    //mov     r0, 0xF0
    cpu.execute_arm(cpu.decode(0xE3A0_00F0));
    r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 0xF0);

    //movs r1,r0,ror 4
    cpu.execute_arm(cpu.decode(0xE1B0_1260));
    r1 = cpu.get_register(1 as u8);
    assert_eq!(r1, 0xF);
    assert!(!cpu.evaluate_cond(Condition::CS));
}

#[test]
fn dp_shift_special() {
    let mut cpu: CPU<Memory> = CPU::new();
    //mov     r0, 0x0
    cpu.execute_arm(cpu.decode(0xE3A0_0000));
    let r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 0x0);

    //msr  cpsr, FLAG_C
    cpu.execute_arm(cpu.decode(0xE328_F202));
    assert!(cpu.psr[cpu.operating_mode].get_c());

    //       movs    r0, r0, rrx E1B00060
    cpu.execute_arm(cpu.decode(0xE1B0_0060));
    let r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 0x8000_0000);
    assert!(!cpu.evaluate_cond(Condition::CS));

    //       cmp     r0, 1 shl 31
    cpu.execute_arm(cpu.decode(0xE350_0102));
    assert!(cpu.psr[cpu.operating_mode].get_z());
}

//TODO: test relativi a PC(t221)(bisogna prima implementare il pipelining,prefetch etc)
