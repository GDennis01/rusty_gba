use std::fmt;
#[derive(Debug)]
pub enum Opcode {
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
pub struct Instruction {
    opc: Opcode,
    data: u32,
    cond: Condition,
}
pub enum Condition {}
impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Opcode::UNDEF => write!(f, "UNDEF"),
            Opcode::AND => write!(f, "AND"),
            Opcode::EOR => write!(f, "EOR"),
            Opcode::SUB => write!(f, "SUB"),
            Opcode::RSB => write!(f, "RSB"),
            Opcode::ADD => write!(f, "ADD"),
            Opcode::ADC => write!(f, "ADC"),
            Opcode::SBC => write!(f, "SBC"),
            Opcode::RSC => write!(f, "RSC"),
            Opcode::TST => write!(f, "TST"),
            Opcode::MRS => write!(f, "MRS"),
            Opcode::MSR => write!(f, "MSR"),
            Opcode::BX => write!(f, "BX"),
            Opcode::B => write!(f, "B"),
            Opcode::MVN => write!(f, "MVN"),
            Opcode::BIC => write!(f, "BIC"),
            Opcode::TEQ => write!(f, "TEQ"),
            Opcode::CMP => write!(f, "CMP"),
            Opcode::CMN => write!(f, "CMN"),
            Opcode::ORR => write!(f, "ORR"),
            Opcode::MOV => write!(f, "MOV"),
            Opcode::STR => write!(f, "STR"),
            Opcode::LDR => write!(f, "LDR"),
            Opcode::LDRH => write!(f, "LDRH"),
            Opcode::STRB => write!(f, "STRB"),
            Opcode::LDRB => write!(f, "LDRB"),
            Opcode::STRH => write!(f, "STRH"),
            Opcode::LDRSB => write!(f, "LDRSB"),
            Opcode::LDM => write!(f, "LDM"),
            Opcode::STM => write!(f, "STM"),
            Opcode::SWI => write!(f, "SWI"),
            Opcode::MUL => write!(f, "MUL"),
            Opcode::LDRSH => write!(f, "LDRSH"),
            Opcode::MLA => write!(f, "MLA"),
            Opcode::UMULL => write!(f, "UMULL"),
            Opcode::UMLAL => write!(f, "UMLAL"),
            Opcode::SMULL => write!(f, "SMULL"),
            Opcode::SMLAL => write!(f, "SMLAL"),
            Opcode::SWP => write!(f, "SWP"),
            Opcode::SWPB => write!(f, "SWPB"),
            Opcode::DBG => write!(f, "DBG"),
        }
    }
}
