pub mod cpu;
pub use cpu::Instruction;
use std::ops::RangeBounds;
pub mod arm32;
pub mod thumb;
use arm32::Arm32;
use thumb::Thumb;

pub trait BitRange {
    fn bit_range<R: RangeBounds<u8>>(&self, range: R) -> Self;
    fn bit(&self, bit: u8) -> bool;
}
impl BitRange for u32 {
    fn bit_range<R: RangeBounds<u8>>(&self, range: R) -> Self {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&n) => n,
            std::ops::Bound::Excluded(&n) => n - 1,
            std::ops::Bound::Unbounded => 0,
        };
        let end: u8 = match range.end_bound() {
            std::ops::Bound::Included(&n) => n,
            std::ops::Bound::Excluded(&n) => n - 1,
            std::ops::Bound::Unbounded => 0,
        };
        (self << (31 - end)) >> (31 - (end - start))
    }
    fn bit(&self, bit: u8) -> bool {
        self.bit_range(bit..=bit) == 1
    }
}
pub enum Mode {
    ARM,
    THUMB,
}
pub struct CPU {
    registers: [u32; 16],
    cpsr: u32,
    spsr: u32,
    pipeline: [u32; 3],
    mode: Mode,
}
impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: [0; 16],
            cpsr: 0,
            spsr: 0,
            pipeline: [0; 3],
            mode: Mode::ARM,
        }
    }
    pub fn decode(&self, instruction: u32) -> Instruction {
        match &self.mode {
            Mode::ARM => Arm32::decode(instruction),
            Mode::THUMB => Thumb::decode(instruction), //REPLACE WITH THUMB
        }
    }
}
