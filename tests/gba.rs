use gba::memory::Memory;
pub fn setup() -> Memory {
    Memory::default()
}
#[cfg(test)]
#[test]
pub fn test_read32() {
    use arm7tdmi::cpu::MemoryInterface;

    let mut mem = Memory::default();
    mem.bios[0] = 0x12;
    mem.bios[1] = 0x34;
    mem.bios[2] = 0x56;
    mem.bios[3] = 0x78;
    let x = mem.read_32(0);
    assert_eq!(x, 0x78563412);
}
