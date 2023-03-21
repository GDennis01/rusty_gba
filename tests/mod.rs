use rusty_gba::arm32::*;

#[cfg(test)]
#[test]
fn test_and() {
    let cpu = Arm32::new();
    match cpu.decode(0xE001_0090) {
        isa::Opcode::MUL => assert!(true),
        _ => assert!(false),
    }

    cpu.decode(0x12345678);
}
