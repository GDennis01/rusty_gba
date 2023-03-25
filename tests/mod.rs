use rusty_gba::*;
#[cfg(test)]
#[test]
fn test_and() {
    let cpu = CPU::new();
    match cpu.decode(0xe59fd1a0) {
        Instruction {
            opc: MRS,
            cond,
            data,
        } => assert!(true),
        _ => assert!(false),
    }
}
