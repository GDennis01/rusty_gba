use crate::{
    arm32::{isa::OpcodeArm, Arm32},
    thumb::{isa::OpcodeThumb, Thumb},
};
use std::fmt;
pub struct Instruction {
    pub opc: Opcode,
    pub data: u32,
    pub cond: Condition,
    // pub fx: FxArm32,
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
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.cond {
            Condition::AL => write!(f, "{:?} {:#06x?}", self.opc, self.data),
            _ => write!(f, "{:?}{:?} {:#06x?}", self.opc, self.cond, self.data),
        }
    }
}

pub enum Opcode {
    Arm32(OpcodeArm),
    Thumb(OpcodeThumb),
}

impl fmt::Debug for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Opcode::Arm32(opc) => write!(f, "{:?}", opc),
            Opcode::Thumb(opc) => write!(f, "{:?}", opc),
        }
    }
}
#[derive(Debug)]
pub enum Condition {
    EQ,
    NE,
    CS,
    CC,
    MI,
    PL,
    VS,
    VC,
    HI,
    LS,
    GE,
    LT,
    GT,
    LE,
    AL,
    ERR,
}
