use std::fmt;

use crate::arm32::isa::Opcode;

pub struct Instruction {
    pub opc: Opcode,
    pub data: u32,
    pub cond: Condition,
    // pub fx: FxArm32,
}
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.cond {
            Condition::AL => write!(f, "{:?} {:x?}", self.opc, self.data),
            _ => write!(f, "{:?}{:?} {:x?}", self.opc, self.cond, self.data),
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
impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Condition::EQ => write!(f, "EQ"),
            Condition::NE => write!(f, "NE"),
            Condition::CS => write!(f, "CS"),
            Condition::CC => write!(f, "CC"),
            Condition::MI => write!(f, "MI"),
            Condition::PL => write!(f, "PL"),
            Condition::VS => write!(f, "VS"),
            Condition::VC => write!(f, "VC"),
            Condition::HI => write!(f, "HI"),
            Condition::LS => write!(f, "LS"),
            Condition::GE => write!(f, "GE"),
            Condition::LT => write!(f, "LT"),
            Condition::GT => write!(f, "GT"),
            Condition::LE => write!(f, "LE"),
            Condition::AL => write!(f, "AL"),
            Condition::ERR => write!(f, "ERR"),
        }
    }
}
