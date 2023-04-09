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
        let result = op1 & op2;
        self.set_register(instruction.bit_range(12..=15) as u8, result);
        if instruction.bit(20) {
            //TODO: set condition code
            //TODO: cercare di capire come implementare il barrel shifter
        }
    }

    pub fn get_op2(&mut self, instruction: u32) -> u32 {
        todo!(); //TODO: operand 2
    }
}
