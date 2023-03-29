pub mod arm32;
pub mod cpu;
pub mod thumb;
use std::ops::RangeBounds;

///Simple trait that allows basic bit manipulation.<br>
pub trait BitRange {
    ///Returns the bits in the specified range
    fn bit_range<R: RangeBounds<u8>>(&self, range: R) -> Self;
    ///Returns whether the specified bit is set or not
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
