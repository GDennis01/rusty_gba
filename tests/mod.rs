use rusty_gba::arm32::*;
use rusty_gba::isa::*;
#[cfg(test)]
#[test]
fn test_and() {
    let cpu = Arm32::new();
    match cpu.decode(0xe59fd1a0) {
        Instruction {
            opc: MRS,
            cond,
            data,
        } => assert!(true),
        _ => assert!(false),
    }
}
