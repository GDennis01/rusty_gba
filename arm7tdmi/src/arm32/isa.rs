use crate::cpu::{Instruction, MemoryInterface, Mode, OperatingMode, CPU};
use crate::{arm32::Arm32, BitRange};
use OpcodeArm::*;
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
    pub fn B(&self, instruction: Instruction) {
        if self.evaluate_cond(instruction.cond) {
            return;
        }
    }
}
// type FxArm32 = fn(&Arm32, u32) -> u8;
//
//TODO:actual isa definitions
