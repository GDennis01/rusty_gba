use crate::cpu::{MemoryInterface, OperatingMode, CPU};
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
    /*****************************
     * BRANCH AND BRANCH EXCHANGE*
     *****************************/

    //TODO: check overflow and add/sub, they might cause bugs in future
    #[allow(non_snake_case)]
    pub fn BX(&mut self, instruction: u32) {
        todo!()
    }

    #[allow(non_snake_case)]
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

    /************************************************
     * DATA PROCESSING INSTRUCTIONS AND PSR TRANSFER*
     ************************************************/
    #[allow(non_snake_case)]
    /// Rd = Operand 1 AND Operand 2
    pub fn AND(&mut self, instruction: u32) {
        let op1 = self.get_register(instruction.bit_range(16..=19) as u8);
        let op2 = self.get_op2(instruction);
        let result = op1 & op2.0;
        self.set_register(instruction.bit_range(12..=15) as u8, result);
        // if bit 20, set condition flags
        if instruction.bit(20) {
            self.psr[self.operating_mode].set_c(op2.1);
            self.set_condition_flags(result as i32, false);
        }
    }
    #[allow(non_snake_case)]
    /// Rd = Operand 1 XOR Operand 2
    pub fn EOR(&mut self, instruction: u32) {
        let op1 = self.get_register(instruction.bit_range(16..=19) as u8);
        let op2 = self.get_op2(instruction);
        let result = op1 ^ op2.0;
        self.set_register(instruction.bit_range(12..=15) as u8, result);
        // if bit 20, set condition flags
        if instruction.bit(20) {
            self.psr[self.operating_mode].set_c(op2.1);
            self.set_condition_flags(result as i32, false);
        }
    }
    #[allow(non_snake_case)]
    /// Rd = Operand 1 - Operand 2
    pub fn SUB(&mut self, instruction: u32) {
        let op1 = self.get_register(instruction.bit_range(16..=19) as u8) as i32;
        let op2 = self.get_op2(instruction);
        let result = op1 - op2.0 as i32;
        let is_overflow = match op1.checked_sub(op2.0 as i32) {
            Some(_) => false,
            None => true,
        };
        self.set_register(instruction.bit_range(12..=15) as u8, result as u32);
        // if bit 20, set condition flags
        if instruction.bit(20) {
            self.psr[self.operating_mode].set_c(op2.1);
            self.set_condition_flags(result, is_overflow);
        }
    }
    #[allow(non_snake_case)]
    ///Reverse SUB, it swaps order of operand 1 and operand 2.
    pub fn RSB(&mut self, instruction: u32) {
        let op1 = self.get_register(instruction.bit_range(16..=19) as u8) as i32;
        let op2 = self.get_op2(instruction);
        let result = op2.0 as i32 - op1;
        let is_overflow = match (op2.0 as i32).checked_sub(op1) {
            Some(_) => false,
            None => true,
        };
        self.set_register(instruction.bit_range(12..=15) as u8, result as u32);
        // if bit 20, set condition flags
        if instruction.bit(20) {
            self.psr[self.operating_mode].set_c(op2.1);
            self.set_condition_flags(result, is_overflow);
        }
    }
    #[allow(non_snake_case)]
    ///Rd = Operand 1 + Operand 2
    pub fn ADD(&mut self, instruction: u32) {
        let op1 = self.get_register(instruction.bit_range(16..=19) as u8) as i32;
        let op2 = self.get_op2(instruction);
        let result = op1 + op2.0 as i32;
        let is_overflow = match op1.checked_add(op2.0 as i32) {
            Some(_) => false,
            None => true,
        };
        self.set_register(instruction.bit_range(12..=15) as u8, result as u32);
        // if bit 20, set condition flags
        if instruction.bit(20) {
            self.psr[self.operating_mode].set_c(op2.1);
            self.set_condition_flags(result, is_overflow);
        }
    }
    #[allow(non_snake_case)]
    ///Rd = Operand 1 + Operand 2 + C flag.
    pub fn ADC(&mut self, instruction: u32) {
        let op1 = self.get_register(instruction.bit_range(16..=19) as u8) as i32;
        let op2 = self.get_op2(instruction);
        let result = op1 + op2.0 as i32 + self.psr[self.operating_mode].get_c() as i32;
        let is_overflow = match op1.checked_add(op2.0 as i32) {
            Some(_) => false,
            None => true,
        };
        self.set_register(instruction.bit_range(12..=15) as u8, result as u32);
        // if bit 20, set condition flags
        if instruction.bit(20) {
            self.psr[self.operating_mode].set_c(op2.1);
            self.set_condition_flags(result, is_overflow);
        }
    }
    #[allow(non_snake_case)]
    ///Rd = Operand 1 - Operand 2 + C flag - 1.
    pub fn SBC(&mut self, instruction: u32) {
        let op1 = self.get_register(instruction.bit_range(16..=19) as u8) as i32;
        let op2 = self.get_op2(instruction);
        let result = op1 - op2.0 as i32 + self.psr[self.operating_mode].get_c() as i32 - 1;
        let is_overflow = match op1.checked_sub(op2.0 as i32) {
            Some(_) => false,
            None => true,
        };
        self.set_register(instruction.bit_range(12..=15) as u8, result as u32);
        // if bit 20, set condition flags
        if instruction.bit(20) {
            self.psr[self.operating_mode].set_c(op2.1);
            self.set_condition_flags(result, is_overflow);
        }
    }
    #[allow(non_snake_case)]
    ///Rd = Operand 2 - Operand 1 + C flag - 1.
    pub fn RSC(&mut self, instruction: u32) {
        let op1 = self.get_register(instruction.bit_range(16..=19) as u8) as i32;
        let op2 = self.get_op2(instruction);

        let result = op2.0 as i32 - op1 + self.psr[self.operating_mode].get_c() as i32 - 1;
        let is_overflow = match (op2.0 as i32).checked_sub(op1) {
            Some(_) => false,
            None => true,
        };
        self.set_register(instruction.bit_range(12..=15) as u8, result as u32);
        // if bit 20, set condition flags
        if instruction.bit(20) {
            self.psr[self.operating_mode].set_c(op2.1);
            self.set_condition_flags(result, is_overflow);
        }
    }
    #[allow(non_snake_case)]
    ///Set condition flags for Operand 1 AND Operand 2.
    pub fn TST(&mut self, instruction: u32) {
        let op1 = self.get_register(instruction.bit_range(16..=19) as u8);
        let op2 = self.get_op2(instruction);
        let result = op1 & op2.0;
        // if bit 20, set condition flags
        if instruction.bit(20) {
            self.psr[self.operating_mode].set_c(op2.1);
            self.set_condition_flags(result as i32, false);
        }
    }
    #[allow(non_snake_case)]
    ///Set condition flags for Operand 1 XOR Operand 2.
    pub fn TEQ(&mut self, instruction: u32) {
        let op1 = self.get_register(instruction.bit_range(16..=19) as u8);
        let op2 = self.get_op2(instruction);
        let result = op1 ^ op2.0;
        // if bit 20, set condition flags
        if instruction.bit(20) {
            self.psr[self.operating_mode].set_c(op2.1);
            self.set_condition_flags(result as i32, false);
        }
    }
    #[allow(non_snake_case)]
    ///Set condition flags for Operand 1 - Operand 2.
    pub fn CMP(&mut self, instruction: u32) {
        let op1 = self.get_register(instruction.bit_range(16..=19) as u8) as i32;
        let op2 = self.get_op2(instruction);
        let result = op1 - op2.0 as i32;
        let is_overflow = match op1.checked_sub(op2.0 as i32) {
            Some(_) => false,
            None => true,
        };
        self.set_register(instruction.bit_range(12..=15) as u8, result as u32);
        // if bit 20, set condition flags
        if instruction.bit(20) {
            self.psr[self.operating_mode].set_c(op2.1);
            self.set_condition_flags(result, is_overflow);
        }
    }
    #[allow(non_snake_case)]
    ///Set condition flags for Operand 1 + Operand 2.
    pub fn CMN(&mut self, instruction: u32) {
        let op1 = self.get_register(instruction.bit_range(16..=19) as u8) as i32;
        let op2 = self.get_op2(instruction);
        let result = op1 + op2.0 as i32;
        let is_overflow = match op1.checked_sub(op2.0 as i32) {
            Some(_) => false,
            None => true,
        };
        // if bit 20, set condition flags
        if instruction.bit(20) {
            self.psr[self.operating_mode].set_c(op2.1);
            self.set_condition_flags(result, is_overflow);
        }
    }
    #[allow(non_snake_case)]
    /// Rd = Operand 1 OR Operand 2
    pub fn ORR(&mut self, instruction: u32) {
        let op1 = self.get_register(instruction.bit_range(16..=19) as u8);
        let op2 = self.get_op2(instruction);
        let result = op1 | op2.0;
        self.set_register(instruction.bit_range(12..=15) as u8, result);
        // if bit 20, set condition flags
        if instruction.bit(20) {
            self.psr[self.operating_mode].set_c(op2.1);
            self.set_condition_flags(result as i32, false);
        }
    }
    #[allow(non_snake_case)]
    /// Rd =  Operand 2
    pub fn MOV(&mut self, instruction: u32) {
        let op2 = self.get_op2(instruction);
        self.set_register(instruction.bit_range(12..=15) as u8, op2.0);
        // if bit 20, set condition flags
        if instruction.bit(20) {
            self.psr[self.operating_mode].set_c(op2.1);
            self.set_condition_flags(op2.0 as i32, false);
        }
    }
    #[allow(non_snake_case)]
    /// Rd = Operand 1 AND NOT Operand 2
    pub fn BIC(&mut self, instruction: u32) {
        let op1 = self.get_register(instruction.bit_range(16..=19) as u8);
        let op2 = self.get_op2(instruction);
        let result = op1 & !op2.0;
        self.set_register(instruction.bit_range(12..=15) as u8, result);
        // if bit 20, set condition flags
        if instruction.bit(20) {
            self.psr[self.operating_mode].set_c(op2.1);
            self.set_condition_flags(result as i32, false);
        }
    }
    #[allow(non_snake_case)]
    /// Rd =  NOT Operand 2
    pub fn MVN(&mut self, instruction: u32) {
        let op2 = self.get_op2(instruction);
        self.set_register(instruction.bit_range(12..=15) as u8, !op2.0);
        // if bit 20, set condition flags
        if instruction.bit(20) {
            self.psr[self.operating_mode].set_c(op2.1);
            self.set_condition_flags(op2.0 as i32, false);
        }
    }

    #[allow(non_snake_case)]
    //TODO: Test thorougly
    /// Transfer (C/S)PSR contents to a specified register<br>
    /// If bit 22 is set, then content is transfered to a SPSR, otherwise just CPSR<br>
    /// When in User/Sys mode, SPSR is considered to be CPSR.
    pub fn MRS(&mut self, instruction: u32) {
        let psr_content: u32 = if instruction.bit(22) {
            self.psr[self.operating_mode].register
        } else {
            self.psr[0].register
        };

        self.set_register(instruction.bit_range(12..=15) as u8, psr_content);
    }

    #[allow(non_snake_case)]
    /// Transfer register content, or immediate value, to (C/S)PSR<br>
    /// This allows to change condition flags as well as control bits(the latter only in priviliged mode)<br>
    /// This instruction might cause bugs since armv4 manual isnt fully detailed about this instruction
    pub fn MSR(&mut self, instruction: u32) {
        let psr_index = if instruction.bit(22) {
            self.operating_mode as usize
        } else {
            0
        };

        match instruction.bit_range(12..=21) {
            //register to psr, if in user mode, only condition bits are set, otherwise whole register is set
            0b10_1001_1111 => {
                let reg_content = self.get_register(instruction.bit_range(0..=3) as u8);

                if let OperatingMode::User = self.operating_mode {
                    self.psr[psr_index].register.set_bits(28..=31, reg_content);
                } else {
                    self.psr[psr_index].register = reg_content;
                }
            }
            //register/immediate value to psr,flag bits only
            0b10_1000_1111 => {
                let imm_value = self.get_immediate_op(instruction).0;
                self.psr[psr_index].register.set_bits(28..=31, imm_value);
            }

            _ => panic!("Error on line {} of Arm/isa.rs", line!()),
        }
    }

    /************************************************
     *            Multiply and Multiply Long        *
     ************************************************/
    #[allow(non_snake_case)]
    /// Computes Rd = Rm * Rs, Rn is ignored<br>
    /// Result is a 32 bit integer<br>
    /// C flag set to meaningless value and V flag unaffected
    fn MUL(&mut self, instruction: u32) {
        let dest = instruction.bit_range(16..=19);
        let rm = self.get_register(instruction.bit_range(0..=3) as u8) as i32;
        let rs = self.get_register(instruction.bit_range(8..=11) as u8) as i32;
        let result = rm.wrapping_mul(rs); //prevents from overflowing
        self.set_register(dest as u8, result as u32);
        // if bit 20, set condition flags
        if instruction.bit(20) {
            self.set_condition_flags(result, false);
            self.psr[self.operating_mode].set_c(false); //random value, it couldve been true
        }
    }
    #[allow(non_snake_case)]
    /// Accumulator form of [`Self::MUL()`] <br>
    /// Computes Rd = Rm * Rs + Rn
    fn MLA(&mut self, instruction: u32) {
        let dest = instruction.bit_range(16..=19);
        let rm = self.get_register(instruction.bit_range(0..=3) as u8) as i32;
        let rs = self.get_register(instruction.bit_range(8..=11) as u8) as i32;
        let rn = self.get_register(instruction.bit_range(12..=15) as u8) as i32;
        let result = rm.wrapping_mul(rs).wrapping_add(rn);
        self.set_register(dest as u8, result as u32);
        // if bit 20, set condition flags
        if instruction.bit(20) {
            self.set_condition_flags(result, false);
            self.psr[self.operating_mode].set_c(false);
        }
    }

    #[allow(non_snake_case)]
    //SMULL and UMULL
    /// Computes a multiplication between 2 32 bit signed integers and produces a 64 bit results<br>
    /// RdHi,RdLo = Rm * Rs where the lower 32 bits of the result are written into RdLo, while upper 32 bits into RdHi.<br>
    /// C and V flags are set to meaningless value.
    fn SMULL(&mut self, instruction: u32) {
        todo!()
    }

    #[allow(non_snake_case)]
    /// Accumulator form of [Self::SMULL()]
    /// RdHi,RdLo = Rm * Rs + RdHi,RdLo. Lower 32 bits of the number to add are read from RdLo, upper 32 bits from RdHi
    fn SMLAL(&mut self, instruction: u32) {
        todo!();
    }

    /// Computes a multiplication between 2 32 bit unsigned integers and produces a 64 bit results<br>
    /// RdHi,RdLo = Rm * Rs where the lower 32 bits of the result are written into RdLo, while upper 32 bits into RdHi.<br>
    /// C and V flags are set to meaningless value.
    fn UMULL(&mut self, instruction: u32) {
        todo!()
    }

    #[allow(non_snake_case)]
    /// Accumulator form of [Self::UMULL()]
    /// RdHi,RdLo = Rm * Rs + RdHi,RdLo. Lower 32 bits of the number to add are read from RdLo, upper 32 bits from RdHi
    fn UMLAL(&mut self, instruction: u32) {
        todo!();
    }

    /// In a data-processing instruction, returns second operand.<br>
    /// Based on bit 21, it can be either an immediate value rotated by a certain amount(bit 21 set) or a shifter register(bit 21 clear)
    fn get_op2(&mut self, instruction: u32) -> (u32, bool) {
        if instruction.bit(25) {
            self.get_immediate_op(instruction)
        } else {
            self.get_shifted_op(instruction)
        }
    }

    /// Helper method to compute immediate value for the second operand<br>
    /// Immediate value is computed as a ROR by twice the value specifed in [8..=12]<br>
    /// Returns a tuple containing immediate value and carry out
    fn get_immediate_op(&mut self, instruction: u32) -> (u32, bool) {
        let imm_value = instruction.bit_range(0..=7);
        let rotate_value = instruction.bit_range(8..=12) * 2;

        self.compute_shift_operation(
            imm_value,
            rotate_value as u8,
            SHIFT::ROR,
            instruction.bit(25),
        )
    }

    /// Helper method to compute shifted register value for the second operand<br>
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
        self.compute_shift_operation(value, amount as u8, shift, false)
    }

    /// Set condition flags based on value
    /// # Arguments
    /// * <b>value:</b> result to check. If <0 its gonna set N flag, if 0 its gonna set Z flag
    /// * <b>overflow:</b> if true, its gonna set V flag
    fn set_condition_flags(&mut self, value: i32, overflow: bool) {
        if value < 0 {
            self.psr[self.operating_mode].set_n(true)
        }
        if value == 0 {
            self.psr[self.operating_mode].set_z(true)
        }
        self.psr[self.operating_mode].set_v(overflow);
    }

    /// Compute the shift operation based on the [`SHIFT`](enum@SHIFT) type<br>
    /// Returns a tuple containing the shifted value and carry out
    /// # Arguments
    /// * **value:** value to be shifted
    /// * **amount:** shift amount to apply on **value**
    /// * **shift:** shift type
    fn compute_shift_operation(
        &mut self,
        value: u32,
        amount: u8,
        shift: SHIFT,
        immediate: bool,
    ) -> (u32, bool) {
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
                // ROR #0 doesnt affect value if dealt with immediate
                if amount == 0 {
                    if !immediate {
                        (
                            ((self.psr[self.operating_mode].get_c() as u32) << 31) | (value >> 1),
                            value.bit(0),
                        )
                    } else {
                        (value, value.bit(0))
                    }
                } else {
                    let overshoot_bits = value.bit_range(0..amount) << (31 - (amount - 1));
                    ((value >> amount) | overshoot_bits, value.bit(amount - 1))
                }
            }
        }
    }

    /// Returns shift type based on a 2 bit value
    /// Panics if value is greater than 4
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
