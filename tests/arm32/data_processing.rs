use arm7tdmi::cpu::*;
use gba::memory::Memory;
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
    assert!(cpu.psr[cpu.operating_mode].get_z()); //Z must be set to 1
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
