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
     * TODO: All dp instructions should behave      *
     *       differently when dealing with r15!     *
     * R15 as Rm/Rn: returned value is PC+12        *
     *               (I=0,R=1 aka shift by register)*
     * shifting by reg takes an extra cycle and     *
     * rn is the last to being read                 *
     *                otherwise PC+8(immediate)     *
     * R15 as Rd   : Save SPSR_curr_mode into CPSR  *
     *               (only if S flag is set),       *
     *                Update PC accordingly to the  *
     *                result of operation,          *
     *                also flushes pipeline         *
     ************************************************/
    /// Copy SPSR into CPSR and flushes the pipeline
    fn handle_r15_as_rd(&mut self) {
        self.psr[0].register = self.psr[self.operating_mode].register;
        self.flush_pipeline();
    }
    /// Wrapping method, used by DP instructions<br>
    /// Sets condition flags if S flag specified and, if it's not CMP/TST etc, <br>
    /// it will set the result to the specified rd register
    /// # Arguments
    /// * **result**: result to eventually set to rd
    /// * **rd**: destination register
    /// * **c:** carry out
    /// * **is_overflow**: overflow
    /// * **is_cond_opc**: if it's CMP/TST etc
    /// * **S**: S flag, if set, sets condition codes
    fn wrap_set_reg_condflags(
        &mut self,
        result: u32,
        rd: u8,
        is_c: bool,
        is_overflow: bool,
        is_cond_opc: bool,
        is_s: bool,
    ) {
        //CMP/TST etc dont set register
        if !is_cond_opc {
            self.set_register(rd, result);
        }
        // if bit 20, set condition flags or move spsrs to cpsr(rd as r15 only)
        if is_s {
            if rd == 15 {
                return self.handle_r15_as_rd();
            }
            self.psr[self.operating_mode].set_c(is_c);
            self.set_condition_flags(result as i32, is_overflow);
        }
    }
    #[allow(non_snake_case)]
    /// Rd = Operand 1 AND Operand 2
    pub fn AND(&mut self, instruction: u32) {
        let rd: u8 = instruction.bit_range(12..=15) as u8;
        let op1 = self.get_op1(instruction);
        let op2 = self.get_op2(instruction);
        let result = op1 & op2.0;
        self.wrap_set_reg_condflags(
            result,
            rd,
            op2.1,
            self.psr[self.operating_mode].get_v(),
            false,
            instruction.bit(20),
        )
    }
    #[allow(non_snake_case)]
    /// Rd = Operand 1 XOR Operand 2
    pub fn EOR(&mut self, instruction: u32) {
        let rd: u8 = instruction.bit_range(12..=15) as u8;
        let op1 = self.get_op1(instruction);
        let op2 = self.get_op2(instruction);
        let result = op1 ^ op2.0;
        self.wrap_set_reg_condflags(
            result,
            rd,
            op2.1,
            self.psr[self.operating_mode].get_v(),
            false,
            instruction.bit(20),
        );
    }
    #[allow(non_snake_case)]
    /// Rd = Operand 1 - Operand 2
    pub fn SUB(&mut self, instruction: u32) {
        let rd: u8 = instruction.bit_range(12..=15) as u8;
        let op1 = self.get_op1(instruction) as i32;
        let op2 = self.get_op2(instruction);
        let result = op1.wrapping_sub(op2.0 as i32);
        let is_overflow = match op1.checked_sub(op2.0 as i32) {
            Some(_) => false,
            None => true,
        };
        self.wrap_set_reg_condflags(
            result as u32,
            rd,
            op2.1,
            is_overflow,
            false,
            instruction.bit(20),
        )
    }

    #[allow(non_snake_case)]
    ///Reverse SUB, it swaps order of operand 1 and operand 2.
    pub fn RSB(&mut self, instruction: u32) {
        let rd: u8 = instruction.bit_range(12..=15) as u8;
        let op1 = self.get_op1(instruction) as i32;
        let op2 = self.get_op2(instruction);
        let result = (op2.0 as i32).wrapping_sub(op1);
        let is_overflow = match (op2.0 as i32).checked_sub(op1) {
            Some(_) => false,
            None => true,
        };
        self.wrap_set_reg_condflags(
            result as u32,
            rd,
            op2.1,
            is_overflow,
            false,
            instruction.bit(20),
        );
    }
    #[allow(non_snake_case)]
    ///Rd = Operand 1 + Operand 2
    pub fn ADD(&mut self, instruction: u32) {
        let rd: u8 = instruction.bit_range(12..=15) as u8;
        let op1 = self.get_op1(instruction) as i32;
        let op2 = self.get_op2(instruction);
        let result = op1.wrapping_add(op2.0 as i32);
        let is_overflow = match op1.checked_add(op2.0 as i32) {
            Some(_) => false,
            None => true,
        };
        self.wrap_set_reg_condflags(
            result as u32,
            rd,
            op2.1,
            is_overflow,
            false,
            instruction.bit(20),
        );
    }
    #[allow(non_snake_case)]
    ///Rd = Operand 1 + Operand 2 + C flag.
    pub fn ADC(&mut self, instruction: u32) {
        let rd: u8 = instruction.bit_range(12..=15) as u8;
        let op1 = self.get_op1(instruction) as i32;
        let op2 = self.get_op2(instruction);
        let result = op1.wrapping_add(op2.0 as i32) + self.psr[self.operating_mode].get_c() as i32;
        let is_overflow = match op1.checked_add(op2.0 as i32) {
            Some(_) => false,
            None => true,
        };
        self.wrap_set_reg_condflags(
            result as u32,
            rd,
            op2.1,
            is_overflow,
            false,
            instruction.bit(20),
        );
    }
    #[allow(non_snake_case)]
    ///Rd = Operand 1 - Operand 2 + C flag - 1.
    pub fn SBC(&mut self, instruction: u32) {
        let rd: u8 = instruction.bit_range(12..=15) as u8;
        let op1 = self.get_op1(instruction) as i32;
        let op2 = self.get_op2(instruction);
        let result =
            op1.wrapping_sub(op2.0 as i32) + self.psr[self.operating_mode].get_c() as i32 - 1;
        let is_overflow = match op1.checked_sub(op2.0 as i32) {
            Some(_) => false,
            None => true,
        };
        self.wrap_set_reg_condflags(
            result as u32,
            rd,
            op2.1,
            is_overflow,
            false,
            instruction.bit(20),
        );
    }
    #[allow(non_snake_case)]
    ///Rd = Operand 2 - Operand 1 + C flag - 1.
    pub fn RSC(&mut self, instruction: u32) {
        let rd: u8 = instruction.bit_range(12..=15) as u8;
        let op1 = self.get_op1(instruction) as i32;
        let op2 = self.get_op2(instruction);

        let result =
            (op2.0 as i32).wrapping_sub(op1) + self.psr[self.operating_mode].get_c() as i32 - 1;
        let is_overflow = match (op2.0 as i32).checked_sub(op1) {
            Some(_) => false,
            None => true,
        };
        self.wrap_set_reg_condflags(
            result as u32,
            rd,
            op2.1,
            is_overflow,
            false,
            instruction.bit(20),
        );
    }
    #[allow(non_snake_case)]
    ///Set condition flags for Operand 1 AND Operand 2.
    pub fn TST(&mut self, instruction: u32) {
        let op1 = self.get_op1(instruction);
        let op2 = self.get_op2(instruction);
        let result = op1 & op2.0;

        self.wrap_set_reg_condflags(
            result as u32,
            0,
            op2.1,
            self.psr[self.operating_mode].get_v(),
            true,
            instruction.bit(20),
        );
    }
    #[allow(non_snake_case)]
    ///Set condition flags for Operand 1 XOR Operand 2.
    pub fn TEQ(&mut self, instruction: u32) {
        let op1 = self.get_op1(instruction);
        let op2 = self.get_op2(instruction);
        let result = op1 ^ op2.0;

        self.wrap_set_reg_condflags(
            result as u32,
            0,
            op2.1,
            self.psr[self.operating_mode].get_v(),
            true,
            instruction.bit(20),
        );
    }
    #[allow(non_snake_case)]
    ///Set condition flags for Operand 1 - Operand 2.
    pub fn CMP(&mut self, instruction: u32) {
        let op1 = self.get_op1(instruction) as i32;
        let op2 = self.get_op2(instruction);
        let result = op1.wrapping_sub(op2.0 as i32);
        let is_overflow = match op1.checked_sub(op2.0 as i32) {
            Some(_) => false,
            None => true,
        };

        self.wrap_set_reg_condflags(
            result as u32,
            0,
            op2.1,
            is_overflow,
            true,
            instruction.bit(20),
        );
    }
    #[allow(non_snake_case)]
    ///Set condition flags for Operand 1 + Operand 2.
    pub fn CMN(&mut self, instruction: u32) {
        let op1 = self.get_op1(instruction) as i32;
        let op2 = self.get_op2(instruction);
        let result = op1.wrapping_add(op2.0 as i32);
        let is_overflow = match op1.checked_add(op2.0 as i32) {
            Some(_) => false,
            None => true,
        };

        self.wrap_set_reg_condflags(
            result as u32,
            0,
            op2.1,
            is_overflow,
            true,
            instruction.bit(20),
        );
    }
    #[allow(non_snake_case)]
    /// Rd = Operand 1 OR Operand 2
    pub fn ORR(&mut self, instruction: u32) {
        let rd: u8 = instruction.bit_range(12..=15) as u8;
        let op1 = self.get_op1(instruction);
        let op2 = self.get_op2(instruction);
        let result = op1 | op2.0;
        self.wrap_set_reg_condflags(
            result as u32,
            rd,
            op2.1,
            self.psr[self.operating_mode].get_v(),
            false,
            instruction.bit(20),
        );
    }
    #[allow(non_snake_case)]
    /// Rd =  Operand 2
    pub fn MOV(&mut self, instruction: u32) {
        let rd: u8 = instruction.bit_range(12..=15) as u8;
        let op2 = self.get_op2(instruction);
        self.wrap_set_reg_condflags(
            op2.0,
            rd,
            op2.1,
            self.psr[self.operating_mode].get_v(),
            false,
            instruction.bit(20),
        );
    }

    #[allow(non_snake_case)]
    /// Rd = Operand 1 AND NOT Operand 2
    pub fn BIC(&mut self, instruction: u32) {
        let rd: u8 = instruction.bit_range(12..=15) as u8;
        let op1 = self.get_op1(instruction);
        let op2 = self.get_op2(instruction);
        let result = op1 & !op2.0;
        self.wrap_set_reg_condflags(
            result as u32,
            rd,
            op2.1,
            self.psr[self.operating_mode].get_v(),
            false,
            instruction.bit(20),
        );
    }
    #[allow(non_snake_case)]
    /// Rd =  NOT Operand 2
    pub fn MVN(&mut self, instruction: u32) {
        let rd: u8 = instruction.bit_range(12..=15) as u8;
        let op2 = self.get_op2(instruction);
        self.wrap_set_reg_condflags(
            !op2.0 as u32,
            rd,
            op2.1,
            self.psr[self.operating_mode].get_v(),
            false,
            instruction.bit(20),
        );
    }

    #[allow(non_snake_case)]
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
    /// This allows to change condition flags as well as control bits(the latter only in privileged mode)<br>
    pub fn MSR(&mut self, instruction: u32) {
        let psr_index = if instruction.bit(22) {
            self.operating_mode
        } else {
            OperatingMode::User
        };
        let data = match instruction.bit(25) {
            true => self.get_immediate_op(instruction).0,
            false => self.get_register(instruction.bit_range(0..=3) as u8),
        };
        if self.operating_mode != OperatingMode::User {
            //control bits
            if instruction.bit(16) {
                self.psr[psr_index].register = self.psr[psr_index].register.set_bits(0..=7, data);
                self.update_operating_mode(instruction.bit(22));
            }
            //reserved
            if instruction.bit(17) {
                self.psr[psr_index].register = self.psr[psr_index].register.set_bits(8..=15, data);
            }
            //reserved
            if instruction.bit(18) {
                self.psr[psr_index].register = self.psr[psr_index].register.set_bits(16..=23, data);
            }
        }
        //flags
        if instruction.bit(19) {
            self.psr[psr_index].register = self.psr[psr_index].register.set_bits(24..=31, data);
        }
    }

    /************************************************
     *            Multiply and Multiply Long        *
     ************************************************/
    #[allow(non_snake_case)]
    /// Computes Rd = Rm * Rs, Rn is ignored<br>
    /// Result is a 32 bit integer<br>
    /// C flag set to meaningless value and V flag unaffected
    pub fn MUL(&mut self, instruction: u32) {
        let dest = instruction.bit_range(16..=19);
        let rm = self.get_register(instruction.bit_range(0..=3) as u8) as i32;
        let rs = self.get_register(instruction.bit_range(8..=11) as u8) as i32;
        let result = rm.wrapping_mul(rs); //prevents from overflowing
        self.set_register(dest as u8, result as u32);
        // if bit 20, set condition flags
        if instruction.bit(20) {
            let v = self.psr[self.operating_mode].get_v();
            let c = self.psr[self.operating_mode].get_c();

            self.set_condition_flags(result, v);
            self.psr[self.operating_mode].set_c(c);
            //random value, it couldve been true
        }
    }
    #[allow(non_snake_case)]
    /// Accumulator form of [`Self::MUL()`] <br>
    /// Computes Rd = Rm * Rs + Rn
    pub fn MLA(&mut self, instruction: u32) {
        let dest = instruction.bit_range(16..=19);
        let rm = self.get_register(instruction.bit_range(0..=3) as u8) as i32;
        let rs = self.get_register(instruction.bit_range(8..=11) as u8) as i32;
        let rn = self.get_register(instruction.bit_range(12..=15) as u8) as i32;
        let result = rm.wrapping_mul(rs).wrapping_add(rn);
        self.set_register(dest as u8, result as u32);
        // if bit 20, set condition flags
        if instruction.bit(20) {
            let v = self.psr[self.operating_mode].get_v();
            let c = self.psr[self.operating_mode].get_c();

            self.set_condition_flags(result, v);
            self.psr[self.operating_mode].set_c(c);
        }
    }

    #[allow(non_snake_case)]
    //SMULL and UMULL
    /// Computes a multiplication between 2 32 bit signed integers and produces a 64 bit results<br>
    /// RdHi,RdLo = Rm * Rs where the lower 32 bits of the result are written into RdLo, while upper 32 bits into RdHi.<br>
    /// C and V flags are set to meaningless value.
    pub fn SMULL(&mut self, instruction: u32) {
        let dest_hi = instruction.bit_range(16..=19);
        let dest_lo = instruction.bit_range(12..=15);

        let rm = self.get_register(instruction.bit_range(0..=3) as u8) as i32 as i64;
        let rs = self.get_register(instruction.bit_range(8..=11) as u8) as i32 as i64;
        let result: i64 = rm.wrapping_mul(rs);

        let result_hi: u32 = (result >> 32) as u32; //upper 32 bits
        let result_lo: u32 = (result & 0xFFFF_FFFF) as u32; //lower 32 bits

        self.set_register(dest_hi as u8, result_hi);
        self.set_register(dest_lo as u8, result_lo);
        // if bit 20, set condition flags
        if instruction.bit(20) {
            let c: bool = self.psr[self.operating_mode].get_c();
            let v: bool = self.psr[self.operating_mode].get_v();
            self.psr[self.operating_mode].set_c(c);
            self.psr[self.operating_mode].set_v(v);

            let z: bool = result_hi == 0 && result_lo == 0;
            let n: bool = (result_hi as u32).bit(31);
            self.psr[self.operating_mode].set_z(z);
            self.psr[self.operating_mode].set_n(n);
        }
    }

    #[allow(non_snake_case)]
    /// Accumulator form of [Self::SMULL()]
    /// RdHi,RdLo = Rm * Rs + RdHi,RdLo. Lower 32 bits of the number to add are read from RdLo, upper 32 bits from RdHi
    pub fn SMLAL(&mut self, instruction: u32) {
        let dest_hi = instruction.bit_range(16..=19);
        let dest_lo = instruction.bit_range(12..=15);

        let rm = self.get_register(instruction.bit_range(0..=3) as u8) as i32 as i64;
        let rs = self.get_register(instruction.bit_range(8..=11) as u8) as i32 as i64;

        //accumulate part
        let dest_hi_val = self.get_register(dest_hi as u8) as i32 as i64;
        let dest_lo_val = self.get_register(dest_lo as u8) as i32 as i64;
        let dest_val: i64 = dest_hi_val << 32 | dest_lo_val;

        let result: i64 = rm.wrapping_mul(rs).wrapping_add(dest_val);
        let result_hi: u32 = (result >> 32) as u32; //upper 32 bits
        let result_lo: u32 = (result & 0xFFFF_FFFF) as u32; //lower 32 bits

        self.set_register(dest_hi as u8, result_hi);
        self.set_register(dest_lo as u8, result_lo);
        // if bit 20, set condition flags
        if instruction.bit(20) {
            let c: bool = self.psr[self.operating_mode].get_c();
            let v: bool = self.psr[self.operating_mode].get_v();
            self.psr[self.operating_mode].set_c(c);
            self.psr[self.operating_mode].set_v(v);

            let z: bool = result_hi == 0 && result_lo == 0;
            let n: bool = (result_hi as u32).bit(31);
            self.psr[self.operating_mode].set_z(z);
            self.psr[self.operating_mode].set_n(n);
        }
    }

    #[allow(non_snake_case)]
    /// Computes a multiplication between 2 32 bit unsigned integers and produces a 64 bit result<br>
    /// RdHi,RdLo = Rm * Rs where the lower 32 bits of the result are written into RdLo, while upper 32 bits into RdHi.<br>
    /// C and V flags are set to meaningless value.
    pub fn UMULL(&mut self, instruction: u32) {
        let dest_hi = instruction.bit_range(16..=19);
        let dest_lo = instruction.bit_range(12..=15);

        let rm = self.get_register(instruction.bit_range(0..=3) as u8) as i32 as u64;
        let rs = self.get_register(instruction.bit_range(8..=11) as u8) as i32 as u64;
        let result: u64 = rm.wrapping_mul(rs);

        let result_hi: u32 = (result >> 32) as u32; //upper 32 bits
        let result_lo: u32 = (result & 0xFFFF_FFFF) as u32; //lower 32 bits

        self.set_register(dest_hi as u8, result_hi);
        self.set_register(dest_lo as u8, result_lo);
        // if bit 20, set condition flags
        if instruction.bit(20) {
            let c: bool = self.psr[self.operating_mode].get_c();
            let v: bool = self.psr[self.operating_mode].get_v();
            self.psr[self.operating_mode].set_c(c);
            self.psr[self.operating_mode].set_v(v);

            let z: bool = result_hi == 0 && result_lo == 0;
            let n: bool = (result_hi as u32).bit(31);
            self.psr[self.operating_mode].set_z(z);
            self.psr[self.operating_mode].set_n(n);
        }
    }

    #[allow(non_snake_case)]
    /// Accumulator form of [Self::UMULL()]
    /// RdHi,RdLo = Rm * Rs + RdHi,RdLo. Lower 32 bits of the number to add are read from RdLo, upper 32 bits from RdHi
    pub fn UMLAL(&mut self, instruction: u32) {
        let dest_hi = instruction.bit_range(16..=19);
        let dest_lo = instruction.bit_range(12..=15);

        let rm = self.get_register(instruction.bit_range(0..=3) as u8) as i32 as u64;
        let rs = self.get_register(instruction.bit_range(8..=11) as u8) as i32 as u64;
        //accumulate part
        let dest_hi_val = self.get_register(dest_hi as u8) as i32 as u64;
        let dest_lo_val = self.get_register(dest_lo as u8) as i32 as u64;
        let dest_val: u64 = dest_hi_val << 32 | dest_lo_val;

        let result: u64 = rm.wrapping_mul(rs).wrapping_add(dest_val);

        let result_hi: u32 = (result >> 32) as u32; //upper 32 bits
        let result_lo: u32 = (result & 0xFFFF_FFFF) as u32; //lower 32 bits

        self.set_register(dest_hi as u8, result_hi);
        self.set_register(dest_lo as u8, result_lo);
        // if bit 20, set condition flags
        if instruction.bit(20) {
            let c: bool = self.psr[self.operating_mode].get_c();
            let v: bool = self.psr[self.operating_mode].get_v();
            self.psr[self.operating_mode].set_c(c);
            self.psr[self.operating_mode].set_v(v);

            let z: bool = result_hi == 0 && result_lo == 0;
            let n: bool = (result_hi as u32).bit(31);
            self.psr[self.operating_mode].set_z(z);
            self.psr[self.operating_mode].set_n(n);
        }
    }

    /************************************************
     *            LDR/STR                           *
     ************************************************/

    #[allow(non_snake_case)]
    /// Load a byte(or a word) from a specified base register(plus/minus a possible shifted offset register).<br>
    /// If specified, modified register can be written back to base register(W flag).<br>
    /// Offset can be added before(pre-indexing) or after(post-indexing) the transfer.<br>
    /// Post-indexing always writes back to base register, thus it's redundant setting W to 1(except for forcing non priviliged mode for transfer)
    pub fn LDR(&mut self, instruction: u32) {
        let base_register = instruction.bit_range(16..=19) as u8;
        let base_register_val = self.get_register(base_register);
        let dest_register = instruction.bit_range(12..=15) as u8;
        //flags
        let is_write_back = instruction.bit(21);
        let is_byte_transfer = instruction.bit(22);
        let is_add = instruction.bit(23);
        let is_post = instruction.bit(24);
        let is_immediate_reg = instruction.bit(25);

        let offset = if is_immediate_reg {
            self.get_shifted_op(instruction).0
        } else {
            instruction.bit_range(0..=12) as u32
        };
        //pre/post-indexing address calculation
        let mut address = if is_add {
            base_register_val + offset
        } else {
            base_register_val - offset
        };
        address = if !is_post { address } else { base_register_val };

        let data = if is_byte_transfer {
            self.memory.read_8(address) as u32
        } else {
            self.read_32_aligned(address)
        };
        if is_post || is_write_back {
            self.set_register(base_register, base_register_val);
        }
        self.set_register(dest_register, data);
    }
    /// Reads a word(32 bit).<br>
    /// If the address is misaligned(i.e., address not a multiple of 4), it gets &'d with !3 to force it to an
    /// aligned address and then ROR data by (addr & 3)*8
    fn read_32_aligned(&mut self, address: u32) -> u32 {
        let data = self.memory.read_32(address & !3);
        self.compute_shift_operation(data, ((address & 3) * 8) as u8, SHIFT::ROR, true)
            .0
    }

    #[allow(non_snake_case)]
    /// Store a byte(or a word) to a specified base register(plus/minus a possible shifted offset register).<br>
    /// If specified, modified register can be written back to base register(W flag).<br>
    /// Offset can be added before(pre-indexing) or after(post-indexing) the transfer.<br>
    /// Post-indexing always writes back to base register, thus it's redundant setting W to 1(except for forcing non priviliged mode for transfer)
    /// Store a byte(or a word)
    /// TODO:
    pub fn STR(&mut self, instruction: u32) {
        let base_register = instruction.bit_range(16..=19) as u8;
        let base_register_val = self.get_register(base_register);
        let dest_register = instruction.bit_range(12..=15) as u8;
        let dest_register_val = self.get_register(dest_register);
        //flags
        let is_write_back = instruction.bit(21);
        let is_byte_transfer = instruction.bit(22);
        let is_add = instruction.bit(23);
        let is_post = instruction.bit(24);
        let is_immediate_reg = instruction.bit(25);

        let offset = if is_immediate_reg {
            self.get_shifted_op(instruction).0
        } else {
            instruction.bit_range(0..=12) as u32
        };
        //pre/post-indexing address calculation
        let mut address = if is_add {
            base_register_val + offset
        } else {
            base_register_val - offset
        };
        address = if !is_post { address } else { base_register_val };

        if is_post || is_write_back {
            self.set_register(base_register, base_register_val);
        }
        if is_byte_transfer {
            self.memory.write_8(address, dest_register_val as u8);
        } else {
            self.memory.write_32(address, dest_register_val);
        }
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

    /// Returns operand 1(rn), which corresponds to bit [16-19]
    /// If rn is 15 and operand 2 is shifted by reg, then PC+12 is returned
    /// Oterwise PC+8
    fn get_op1(&mut self, instruction: u32) -> u32 {
        let rn = instruction.bit_range(16..=19) as u8;
        if rn == 15 {
            if !instruction.bit(4) {
                return self.registers[15] + 8;
            } else {
                return self.registers[15] + 12;
            }
        }
        self.get_register(rn)
    }

    /// Helper method to compute immediate value for the second operand<br>
    /// Immediate value is computed as a ROR by twice the value specifed in [8..=11]<br>
    /// Returns a tuple containing immediate value and carry out
    fn get_immediate_op(&mut self, instruction: u32) -> (u32, bool) {
        let imm_value = instruction.bit_range(0..=7);
        let rotate_value = instruction.bit_range(8..=11) * 2;

        self.compute_shift_operation(
            imm_value,
            rotate_value as u8,
            SHIFT::ROR,
            instruction.bit(25),
        )
    }

    /// Helper method to compute shifted register value for the second operand<br>
    /// The shifted amount can be either an immediate value or bottom byte of a specified register
    /// If rm is r15,then it is returned either r15+8(if immediate) or r15+12
    /// Returns a tuple containing the value and carry out
    fn get_shifted_op(&mut self, instruction: u32) -> (u32, bool) {
        let shift: SHIFT = self.get_shift(instruction.bit_range(5..=6));
        let rm: u8 = instruction.bit_range(0..=3) as u8;
        let value: u32 = self.get_register(rm);
        let amount: u32;

        // if bit 4 is clear, then the shifted amount is an immediate value
        if !instruction.bit(4) {
            amount = instruction.bit_range(7..=11);
            if rm == 15 {
                return (self.registers[15] + 8, false);
            }
        } else if instruction.bit(4) && !instruction.bit(7) {
            if rm == 15 {
                return (self.registers[15] + 12, false);
            }
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
                        (value, self.psr[self.operating_mode].get_c())
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
