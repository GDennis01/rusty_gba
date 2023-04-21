use arm7tdmi::cpu::*;
use gba::memory::Memory;
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
    cpu.psr[cpu.operating_mode].register = 0xF000_0000;

    //msr cpsr_f, 0xF0000000 -> set flags to 1
    cpu.execute_arm(cpu.decode(0xF0000000));

    assert!(cpu.psr[cpu.operating_mode].get_c());
    assert!(cpu.psr[cpu.operating_mode].get_z());
    assert!(cpu.psr[cpu.operating_mode].get_v());
    assert!(cpu.psr[cpu.operating_mode].get_n());
}

#[test]
fn write_control_bits() {
    //TODO: da finire
    //msr     cpsr_c, MODE_FIQ(0x11) 0xE321_F011
    // mrs     r0, cpsr
    // and     r0, 0x1F
    // cmp     r0, MODE_FIQ(0x11)
    // bne     f252

    // msr     cpsr_c, MODE_SYS(0x1F)

    let mut cpu: CPU<Memory> = CPU::new();
    cpu.psr[cpu.operating_mode].register = 0xF000_0000;

    //msr cpsr_f, 0xF0000000 -> set flags to 1
    cpu.execute_arm(cpu.decode(0xF0000000));

    assert!(cpu.psr[cpu.operating_mode].get_c());
    assert!(cpu.psr[cpu.operating_mode].get_z());
    assert!(cpu.psr[cpu.operating_mode].get_v());
    assert!(cpu.psr[cpu.operating_mode].get_n());
}
