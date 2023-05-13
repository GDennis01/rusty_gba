use arm7tdmi::cpu::*;
use gba::memory::Memory;
/// Tests provided by https://github.com/jsmolka/gba-tests/blob/master/arm/psr_transfer.asm and
/// decoded,instruction by instruction, through https://shell-storm.org/online/Online-Assembler-and-Disassembler/?inst=cmp+r0%2C0x11&arch=arm&as_format=inline#assembly
#[cfg(test)]
#[test]
fn read_write_psr() {
    let mut cpu: CPU<Memory> = CPU::new();
    cpu.psr[cpu.operating_mode].register = 0xF000_0000;

    //mrs r0,cpsr -> move cpsr to r0
    cpu.execute_arm(cpu.decode(0xE10F_0000));
    let r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 0xF000_0000); //r0 must be 0xF000_0000

    //bic r0,0xF0000000 -> clear condition field
    cpu.execute_arm(cpu.decode(0xE3C0_020F));
    let r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 0x0); //r0 must be 0xF

    //msr cpsr,r0 -> move r0 back to cpsr
    cpu.execute_arm(cpu.decode(0xE129_F000));
    assert_eq!(cpu.psr[cpu.operating_mode].register, 0x0); //cpsr must be 0x0

    assert!(!cpu.psr[cpu.operating_mode].get_c());
    assert!(!cpu.psr[cpu.operating_mode].get_z());
    assert!(!cpu.psr[cpu.operating_mode].get_v());
    assert!(!cpu.psr[cpu.operating_mode].get_n());
}

#[test]
fn write_flag_bits() {
    let mut cpu: CPU<Memory> = CPU::new();
    cpu.psr[cpu.operating_mode].register = 0x0000_0000;

    //msr cpsr_f, 0xF0000000 -> set flags to 1
    cpu.execute_arm(cpu.decode(0xE328_F20F));

    assert!(cpu.psr[cpu.operating_mode].get_c());
    assert!(cpu.psr[cpu.operating_mode].get_z());
    assert!(cpu.psr[cpu.operating_mode].get_v());
    assert!(cpu.psr[cpu.operating_mode].get_n());
}

#[test]
fn write_control_bits() {
    let mut cpu: CPU<Memory> = CPU::new();
    // to change cpsr_c I must be in a privileged mode, thus I switch first to SYS
    cpu.psr[0].register = 0x1F; //SYS
    cpu.update_operating_mode(false);
    match cpu.operating_mode {
        OperatingMode::System => assert!(true),
        _ => assert!(false),
    }

    //msr cpsr_c, 0x11(MODE_FIQ) 0xE321_F011
    cpu.execute_arm(cpu.decode(0xE321_F011));
    match cpu.operating_mode {
        OperatingMode::FIQ => assert!(true),
        _ => assert!(false),
    }

    // mrs r0, cpsr
    cpu.execute_arm(cpu.decode(0xE10F_0000));
    let r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 0x11);

    // and r0, 0x1F
    cpu.execute_arm(cpu.decode(0xE200_001F));
    let r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 0x11);

    // cmp r0, MODE_FIQ(0x11)
    cpu.execute_arm(cpu.decode(0xE350_0011));
    assert!(cpu.psr[cpu.operating_mode].get_z())
}

#[test]
fn register_banking() {
    let mut cpu: CPU<Memory> = CPU::new();
    // to change cpsr_c I must be in a privileged mode, thus I switch first to SYS
    cpu.psr[0].register = 0x1F;
    cpu.update_operating_mode(false);
    match cpu.operating_mode {
        OperatingMode::System => assert!(true),
        _ => assert!(false),
    }

    //mov r0,16
    cpu.execute_arm(cpu.decode(0xE3A0_0010));
    let mut r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 16); //r0 must be 16

    //mov r8,32
    cpu.execute_arm(cpu.decode(0xE3A8_8020));
    let mut r8 = cpu.get_register(8 as u8);
    assert_eq!(r8, 32); //r0 must be 32

    //msr cpsr_c, 0x11(MODE_FIQ)
    cpu.execute_arm(cpu.decode(0xE321_F011));
    match cpu.operating_mode {
        OperatingMode::FIQ => assert!(true),
        _ => assert!(false),
    }

    // mov     r0, 32
    cpu.execute_arm(cpu.decode(0xE3A0_0020));
    r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 32); //r0 must be 32

    // mov     r8, 64
    cpu.execute_arm(cpu.decode(0xE3A8_8040));
    r8 = cpu.get_register(8 as u8);
    assert_eq!(r8, 64); //r0 must be 64

    // msr     cpsr_c, MODE_SYS
    cpu.execute_arm(cpu.decode(0xE321_F01F));
    match cpu.operating_mode {
        OperatingMode::System => assert!(true),
        _ => assert!(false),
    }

    //cmp r0,32
    cpu.execute_arm(cpu.decode(0xE350_0020));
    assert!(cpu.psr[cpu.operating_mode].get_z());

    //cmp r8,32
    cpu.execute_arm(cpu.decode(0xE358_0020));
    assert!(cpu.psr[cpu.operating_mode].get_z())
}

#[test]
fn accessing_psr() {
    let mut cpu: CPU<Memory> = CPU::new();
    cpu.psr[cpu.operating_mode].register = 0xF0F0_F0FF; //random value

    //  mrs     r0, cpsr
    cpu.execute_arm(cpu.decode(0xE10F_0000));
    let r0 = cpu.get_register(0 as u8);
    assert_eq!(r0, 0xF0F0_F0FF); //r0 must be 0xF0F0_F0FF

    //  msr     spsr, r0
    cpu.execute_arm(cpu.decode(0xE169_F000));
    assert_eq!(cpu.psr[cpu.operating_mode].register, 0xF0F0_F0FF);

    //  mrs     r1, spsr 0xE14F_1000
    cpu.execute_arm(cpu.decode(0xE14F_1000));
    let r1 = cpu.get_register(1 as u8);
    assert_eq!(r1, 0xF0F0_F0FF);

    //  cmp     r1, r0   0xE151_0000
    cpu.execute_arm(cpu.decode(0xE151_0000));
    assert!(cpu.psr[cpu.operating_mode].get_z());
}
