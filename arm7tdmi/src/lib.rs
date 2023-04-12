pub mod arm32;
pub mod cpu;
pub mod thumb;
use std::ops::{BitOrAssign, RangeBounds};

///Simple trait that allows basic bit manipulation.<br>
pub trait BitRange {
    ///Returns the bits in the specified range
    fn bit_range<R: RangeBounds<u8>>(&self, range: R) -> Self;
    ///Returns whether the specified bit is set or not
    fn bit(&self, bit: u8) -> bool;
    ///Sets the specified bits to the given value in the specified range.<br>
    ///So, to set the x-th bit to `1`, you will have to provide a `u32` with a `1` in the x-th position
    /// # Example
    /// `0x1234_5678.set_bits(20..=24,0x9ABC_DEF0)` will sets bits 0x34 to 0xBC
    fn set_bits<R: RangeBounds<u8>>(&self, range: R, data: u32) -> Self;
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
            std::ops::Bound::Unbounded => 31,
        };
        (self << (31 - end)) >> (31 - (end - start))
    }
    fn bit(&self, bit: u8) -> bool {
        self.bit_range(bit..=bit) == 1
    }

    fn set_bits<R: RangeBounds<u8>>(&self, range: R, data: u32) -> Self {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&n) => n,
            std::ops::Bound::Excluded(&n) => n - 1,
            std::ops::Bound::Unbounded => 0,
        };
        let end: u8 = match range.end_bound() {
            std::ops::Bound::Included(&n) => n,
            std::ops::Bound::Excluded(&n) => n - 1,
            std::ops::Bound::Unbounded => 31,
        };

        //TODO: capire come far funzionare sta roba
        //left part of data i.e. 0x1000_0001 with range 7-8
        let msb_self = if end < 31 {
            self.bit_range(end + 1..) >> start
        } else {
            0
        };
        let lsb_self: u32 = if start > 0 {
            self.bit_range(0..start - 1) >> (start - (end - start))
        } else {
            0
        };
        let masked_self = msb_self | lsb_self;
        let tmp: u32 = data.bit_range(start..=end) << start;
        // tmp | (self << (end - start + 1) >> (end - start + 1))
        tmp | masked_self
    }
}
