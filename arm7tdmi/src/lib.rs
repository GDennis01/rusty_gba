pub mod arm32;
pub mod cpu;
pub mod thumb;
use std::ops::RangeBounds;

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
