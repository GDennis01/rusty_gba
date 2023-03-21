use rusty_gba::arm32::*;

#[cfg(test)]
#[test]
fn test_and() {
    let cpu = Arm32::new();
    match cpu.decode(0x120123) {
        isa::Opcode::AND => assert!(true),
        _ => assert!(false),
    }

    cpu.decode(0x12345678);
}
