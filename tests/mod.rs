pub mod cpu;
pub mod gba;
use arm7tdmi::BitRange;
#[cfg(test)]
#[test]
fn set_bits() {
    let mut data: u32 = 0x1234_5678;
    let value: u32 = 0x9ABC_DEF0;
    data = data.set_bits(28..=31, value);
    assert_eq!(data, 0x9234_5678);
}
#[test]
fn set_bits2() {
    let mut data: u32 = 0x1234_5678;
    let value: u32 = 0x07C0_0000;
    data = data.set_bits(22..=26, value);
    assert_eq!(data, 0x17F4_5678);
}
#[test]
fn set_bits3() {
    let mut data: u32 = 0x27E5_3F10;
    let value: u32 = 0x49DC_7D6A;
    data = data.set_bits(11..=19, value);
    assert_eq!(data, 0x27EC_7F10);
}
#[test]
fn set_bits4() {
    let mut data: u32 = 0x283E_B7ED;
    let value: u32 = 0x1BB0_CB70;
    data = data.set_bits(7..=25, value);
    assert_eq!(data, 0x2BB0_CB6D);
}
#[test]
fn set_bits_full() {
    let mut data: u32 = 0x283E_B7ED;
    let value: u32 = 0x1BB0_CB70;
    data = data.set_bits(0..=31, value);
    assert_eq!(data, 0x1BB0_CB70);
}
#[test]
fn set_bits_startzero() {
    let mut data: u32 = 0x283E_B7ED;
    let value: u32 = 0x1BB0_CB70;
    data = data.set_bits(0..=7, value);
    assert_eq!(data, 0x283E_B770);
}
#[test]
fn bit_range() {
    let value: u32 = 0x1234_5678;

    assert_eq!(value.bit_range(28..=31), 0x1)
}
#[test]
fn bit_range2() {
    let value: u32 = 0x9ABE_DEFC;

    assert_eq!(value.bit_range(17..=19), 0b111)
}
#[test]
fn bit_range3() {
    let value: u32 = 0x9ABE_DEFC;

    assert_eq!(value.bit_range(31..), 0b1)
}
