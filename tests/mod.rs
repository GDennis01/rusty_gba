pub mod cpu;
pub mod gba;
use arm7tdmi::BitRange;
#[cfg(test)]
#[test]
fn test_set_bits() {
    let mut data: u32 = 0x1234_5678;
    let value: u32 = 0x9ABC_DEF0;
    data = data.set_bits(28..=31, value);
    println!("{:#X}", data);
    assert_eq!(data, 0x9234_5678);
}
#[test]
fn test_set_bits2() {
    let mut data: u32 = 0x1234_5678;
    let value: u32 = 0x07C0_0000;
    println!("{:#X}", data.bit_range(22..=26));
    data = data.set_bits(22..=26, value);
    println!("{:#X}", data);
    assert_eq!(data, 0x17F4_5678);
}
#[test]
fn test_bit_range() {
    let value: u32 = 0x1234_5678;

    assert_eq!(value.bit_range(28..=31), 0x1)
}
#[test]
fn test_bit_range2() {
    let value: u32 = 0x9ABE_DEFC;

    assert_eq!(value.bit_range(17..=19), 0b111)
}
#[test]
fn test_bit_range3() {
    let value: u32 = 0x9ABE_DEFC;

    assert_eq!(value.bit_range(31..), 0b1)
}
