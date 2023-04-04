use gba::memory::Memory;
#[cfg(test)]
#[test]
pub fn read8() {
    use arm7tdmi::cpu::MemoryInterface;

    let mut mem = Memory::default();
    mem.bios[0] = 0x12;
    let x = mem.read_8(0);
    assert_eq!(x, 0x12);
}
#[test]
pub fn read16() {
    use arm7tdmi::cpu::MemoryInterface;

    let mut mem = Memory::default();
    mem.bios[0] = 0x12;
    mem.bios[1] = 0x34;
    let x = mem.read_16(0);
    assert_eq!(x, 0x3412);
}
#[test]
pub fn read32() {
    use arm7tdmi::cpu::MemoryInterface;

    let mut mem = Memory::default();
    mem.bios[0] = 0x12;
    mem.bios[1] = 0x34;
    mem.bios[2] = 0x56;
    mem.bios[3] = 0x78;
    let x = mem.read_32(0);
    assert_eq!(x, 0x78563412);
}

#[test]
pub fn write8() {
    use arm7tdmi::cpu::MemoryInterface;

    let mut mem = Memory::default();
    mem.bios[0] = 0xFF;
    mem.bios[1] = 0x34;
    mem.write_8(0, 0x12);
    assert_eq!(mem.bios[0], 0x12);
    assert_eq!(mem.bios[1], 0x34);
}
#[test]
pub fn write16() {
    use arm7tdmi::cpu::MemoryInterface;

    let mut mem = Memory::default();
    mem.bios[0] = 0xFF;
    mem.bios[1] = 0xFF;
    mem.bios[2] = 0xFF;
    mem.bios[3] = 0xFF;
    mem.write_16(0, 0x1234);

    assert_eq!(mem.bios[0], 0x34);
    assert_eq!(mem.bios[1], 0x12);
}
#[test]
pub fn write32() {
    use arm7tdmi::cpu::MemoryInterface;

    let mut mem = Memory::default();
    mem.bios[0] = 0xFF;
    mem.bios[1] = 0xFF;
    mem.bios[2] = 0xFF;
    mem.bios[3] = 0xFF;
    mem.write_32(0, 0x12345678);

    assert_eq!(mem.bios[0], 0x78);
    assert_eq!(mem.bios[1], 0x56);
    assert_eq!(mem.bios[2], 0x34);
    assert_eq!(mem.bios[3], 0x12);
}
