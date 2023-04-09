use crate::cpu::{MemoryInterface, CPU};
use crate::BitRange;
#[derive(Debug)]
pub enum OpcodeArm {
    ADC,
    ADD,
    AND,
    B,
    BIC,
    BX,
    CMN,
    CMP,
    EOR,
    LDM,
    LDR,
    LDRB,
    LDRH,
    LDRSB,
    LDRSH,
    MLA,
    MOV,
    MRS,
    MSR,
    MUL,
    MVN,
    ORR,
    RSB,
    RSC,
    SBC,
    SMLAL,
    SMULL,
    STM,
    STR,
    STRB,
    STRH,
    SUB,
    SWI,
    SWP,
    SWPB,
    TEQ,
    TST,
    UMLAL,
    UMULL,
    UNDEF,
    DBG,
}
impl<T: MemoryInterface + Default> CPU<T> {
    pub fn BX(&mut self, instruction: u32) {
        todo!()
    }

    /// Adds a signed 2 complement 24 bit offset(shitfted left by 2) to PC
    /// If Link bit is set, overwrites Link Register of current bank with PC
    /// Cycles: 2S + 1N
    pub fn B(&mut self, instruction: u32) {
        //Link bit set
        if instruction.bit(24) {
            let pc = self.get_register(15);
            self.set_register(14, pc);
        }
        let offset = (instruction.bit_range(0..=23) as i32) << 2;
        self.registers[15] = (self.registers[15] as i32 + offset) as u32;
    }

    pub fn AND(&mut self, instruction: u32) {
        let op1 = self.get_register(instruction.bit_range(16..=19) as u8);
        let op2 = self.get_op2(instruction);
        let result = op1 & op2.0;
        self.set_register(instruction.bit_range(12..=15) as u8, result);
        // if bit 20, set condition flags
        if instruction.bit(20) {
            self.psr[self.operating_mode].set_c(op2.1);
            self.set_condition_flags(result);
        }
    }

    /// In a data-processing instruction, returns second operand.<br>
    /// Based on bit 21, it can be either an immediate value rotated by a certain amount(bit 21 set) or a shifter register(bit 21 clear)
    fn get_op2(&mut self, instruction: u32) -> (u32, bool) {
        if instruction.bit(21) {
            self.get_immediate_op(instruction)
        } else {
            self.get_shifted_op(instruction)
        }
    }

    /// Helper method to compute immediate value for the second operand
    /// Immediate value is computed as a ROR by twice the value specifed in [8..=12]
    fn get_immediate_op(&mut self, instruction: u32) -> (u32, bool) {
        let imm_value = instruction.bit_range(0..=7);
        let rotate_value = instruction.bit_range(8..=12) * 2;
        self.compute_shift_operation(imm_value, rotate_value as u8, SHIFT::ROR)
    }

    /// Helper method to compute shifted register value for the second operand
    /// The shifted amount can be either an immediate value or bottom byte of a specified register
    fn get_shifted_op(&mut self, instruction: u32) -> (u32, bool) {
        let shift: SHIFT = self.get_shift(instruction.bit_range(5..=6));
        let value: u32 = self.get_register(instruction.bit_range(0..=3) as u8);
        let amount: u32;

        // if bit 4 is clear, then the shifted amount is an immediate value
        if !instruction.bit(4) {
            amount = instruction.bit_range(7..=11);
        } else if instruction.bit(4) && !instruction.bit(7) {
            //if bit 4 is set and bit 7 is clear,shifted amount is bottom byte of a register
            amount = self.get_register(instruction.bit_range(8..=11) as u8) & 0xFF;
        } else {
            //this should never happen
            panic!("Error on line {} of Arm/isa.rs", line!());
        }
        self.compute_shift_operation(value, amount as u8, shift)
    }

    fn set_condition_flags(&mut self, value: u32) {
        if (value as i32) < 0 {
            self.psr[self.operating_mode].set_n(true)
        } else if value == 0 {
            self.psr[self.operating_mode].set_z(true)
        }
    }

    /// Compute the shift operation based on the [`SHIFT`](enum@SHIFT) type
    /// Returns a tuple containing the shifted value and carry out
    /// # Arguments
    /// * **value:** value to be shifted
    /// * **amount:** shift amount to apply on **value**
    /// * **shift:** shift type
    fn compute_shift_operation(&mut self, value: u32, amount: u8, shift: SHIFT) -> (u32, bool) {
        match shift {
            // LSL #5 -> bits 27-31 are discarded and bit 27 will be carry out
            SHIFT::LSL => {
                // LSL #0 returns the unmodified value and carry out is CPSR's C flag
                if amount == 0 {
                    (value, self.psr[self.operating_mode].get_c())
                } else {
                    (value << amount, value.bit(31 - (amount - 1)))
                }
            }
            // LSR #5 -> bit 0-4 are discarded and bit 4 will be carry out
            SHIFT::LSR => {
                // LSR #0 returns 0 with bit 31 of value as carry out
                if amount == 0 {
                    (0, value.bit(31))
                } else {
                    (value << amount, value.bit(amount - 1))
                }
            }
            // Rust's >> is arithmetic if used with signed, thus I cast value as signed,
            // apply a >> and then re-cast it to unsigned. Carry out is computed in the same way as LSR
            SHIFT::ASR => {
                // ASR #0 returns 0xFFFF_FFFF if bit 31 of value is set,0 otherwise. Bit 31 of value is carry out
                if amount == 0 {
                    (if value.bit(31) { 0xFFFF_FFFF } else { 0 }, value.bit(31))
                } else {
                    (((value as i32) >> amount) as u32, value.bit(amount - 1))
                }
            }
            // ROR #5 -> bits 0-4 becomes bit 27-31, bit 31 becomes bit 26,bit 5 becomes bit 0,bit 4 carry out
            SHIFT::ROR => {
                // ROR #0 is used to encode RRX: result is shifted right of 1 and bit 31 of result is C's flag
                // bit 0 of value is carry out
                if amount == 0 {
                    (
                        ((self.psr[self.operating_mode].get_c() as u32) << 31) | (value >> 1),
                        value.bit(0),
                    )
                } else {
                    let overshoot_bits = value.bit_range(0..amount) << (31 - (amount - 1));
                    ((value >> amount) | overshoot_bits, value.bit(amount - 1))
                }
            }
        }
    }
    fn get_shift(&mut self, value: u32) -> SHIFT {
        match value {
            0b00 => SHIFT::LSL,
            0b01 => SHIFT::LSR,
            0b10 => SHIFT::ASR,
            0b11 => SHIFT::ROR,
            _ => panic!("Error shift"),
        }
    }
}

/// Simple enum containing shift type
enum SHIFT {
    /// Logical Shift Left
    LSL,
    /// Logical Shift Right
    LSR,
    /// Arithmetic Shift Right
    ASR,
    /// Rotate Right
    ROR,
}
