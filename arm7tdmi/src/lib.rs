#![no_std]
extern crate alloc;
extern crate core;
// TODO: make this an external library

pub mod arm32;
pub mod cpu;
pub mod thumb;
// use std::ops::{BitOrAssign, RangeBounds};
use core::ops::RangeBounds;

/// A library for simple bit manipulation.

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
            core::ops::Bound::Included(&n) => n,
            core::ops::Bound::Excluded(&n) => n - 1,
            core::ops::Bound::Unbounded => 0,
        };
        let end: u8 = match range.end_bound() {
            core::ops::Bound::Included(&n) => n,
            core::ops::Bound::Excluded(&n) => n - 1,
            core::ops::Bound::Unbounded => 31,
        };
        (self << (31 - end)) >> (31 - (end - start))
    }
    fn bit(&self, bit: u8) -> bool {
        self.bit_range(bit..=bit) == 1
    }

    fn set_bits<R: RangeBounds<u8>>(&self, range: R, data: u32) -> Self {
        let start = match range.start_bound() {
            core::ops::Bound::Included(&n) => n,
            core::ops::Bound::Excluded(&n) => n - 1,
            core::ops::Bound::Unbounded => 0,
        };
        let end: u8 = match range.end_bound() {
            core::ops::Bound::Included(&n) => n,
            core::ops::Bound::Excluded(&n) => n - 1,
            core::ops::Bound::Unbounded => 31,
        };

        //I destruct the number in: bits on the left of the range(MSB) and bits on the right on the range(LSB)
        //0010 0111 1110 0101 0011 1111 0001 0000
        //**** **** ****^^^^ ^^^^ ^---- ---- ----
        // "*" bits are MSB, "-" bits are LSB, "^" bits are the one selected by the specified range
        let msb_self = if end < 31 {
            self.bit_range(end + 1..) << (end + 1)
        } else {
            0
        };

        let lsb_self: u32 = if start > 0 {
            self.bit_range(0..=start - 1)
        } else {
            0
        };
        let masked_self = msb_self | lsb_self;
        let tmp: u32 = data.bit_range(start..=end) << start;
        // println!("MASKED:{:#034b}", masked_self);
        // println!("TMP   :{:#034b}", tmp);
        let res = tmp | masked_self;
        // println!("RES   :{:#034b}", res);

        res
    }
}
