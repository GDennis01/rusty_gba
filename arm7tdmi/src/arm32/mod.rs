//TODO: Implementare ciclo fetch decode execute
pub mod isa;
use crate::cpu::{
    Condition::{self, *},
    Instruction,
    Opcode::*,
};
use crate::BitRange;
use isa::OpcodeArm::{self, *};
pub struct Arm32 {}

impl Arm32 {
    fn decode_block_data_transfer(instruction: u32) -> OpcodeArm {
        match instruction.bit(20) {
            true => LDM,
            false => STM,
        }
    }
    fn decode_single_data_transfer(instruction: u32) -> OpcodeArm {
        match instruction.bit(20) {
            true => LDR,
            false => STR,
        }
    }
    fn decode_psr(instruction: u32, opc: OpcodeArm) -> OpcodeArm {
        match opc {
            MRS => {
                if instruction.bit_range(16..=21) == 0b001111 && instruction.bit_range(0..12) == 0 {
                    MRS
                } else {
                    UNDEF
                }
            }
            MSR => {
                if instruction.bit_range(20..=21) == 0b10
                    && instruction.bit_range(12..=15) == 0b1111
                {
                    MSR
                } else {
                    UNDEF
                }
            }
            _ => UNDEF,
        }
    }
    fn decode_data_processing_psr_transfer(instruction: u32) -> OpcodeArm {
        match instruction.bit_range(21..=24) {
            0b0000 => AND,
            0b0001 => EOR,
            0b0010 => SUB,
            0b0011 => RSB,
            0b0100 => ADD,
            0b0101 => ADC,
            0b0110 => SBC,
            0b0111 => RSC,
            0b1000 | 0b1010 => {
                return if !instruction.bit(20) {
                    Arm32::decode_psr(instruction, MRS)
                } else {
                    TST
                }
            }
            0b1001 | 0b1011 => {
                return if !instruction.bit(20) {
                    Arm32::decode_psr(instruction, MSR)
                } else {
                    TEQ
                }
            }
            0b1100 => ORR,
            0b1101 => MOV,
            0b1110 => BIC,
            0b1111 => MVN,
            _ => UNDEF,
        }
    }
    fn decode_mul(instruction: u32) -> OpcodeArm {
        match instruction.bit(21) {
            true => MLA,
            false => MUL,
        }
    }
    fn decode_mul_long(instruction: u32) -> OpcodeArm {
        let bit21 = instruction.bit(21);
        match instruction.bit(22) {
            true => match bit21 {
                true => SMLAL,
                false => SMULL,
            },
            false => match bit21 {
                true => UMLAL,
                false => UMULL,
            },
        }
    }
    fn decode_hdt(instruction: u32) -> OpcodeArm {
        let r = instruction.bit_range(5..=6);
        match instruction.bit(20) {
            true => match r {
                0b00 => UNDEF,
                0b01 => LDRH,
                0b10 => LDRSB,
                0b11 => LDRSH,
                _ => UNDEF,
            },
            false => match r {
                0b00 => UNDEF,
                0b01 => STRH,
                0b10 => UNDEF,
                0b11 => UNDEF,
                _ => UNDEF,
            },
        }
    }
    fn get_condition(instruction: u32) -> Condition {
        return match instruction.bit_range(28..=31) {
            0b0000 => EQ,
            0b0001 => NE,
            0b0010 => CS,
            0b0011 => CC,
            0b0100 => MI,
            0b0101 => PL,
            0b0110 => VS,
            0b0111 => VC,
            0b1000 => HI,
            0b1001 => LS,
            0b1010 => GE,
            0b1011 => LT,
            0b1100 => GT,
            0b1101 => LE,
            0b1110 => AL,
            0b1111 => ERR,
            _ => ERR,
        };
    }
    pub fn decode(instruction: u32) -> Instruction {
        let cond: Condition = Arm32::get_condition(instruction);
        if instruction.bit_range(4..=27) == 0x12FFF1 {
            return Instruction {
                opc: Arm32(BX),
                data: instruction,
                cond,
            };
        }
        if instruction.bit_range(24..=27) == 0b1111 {
            return Instruction {
                opc: Arm32(SWI),
                data: instruction,
                cond,
            };
        }
        match instruction.bit_range(25..28) {
            0b011 if instruction.bit(4) => Instruction {
                opc: Arm32(UNDEF),
                data: instruction,
                cond,
            },
            0b101 => Instruction {
                opc: Arm32(B),
                data: instruction,
                cond,
            },
            0b100 => Instruction {
                opc: Arm32(Arm32::decode_block_data_transfer(instruction)),
                data: instruction,
                cond,
            },
            0b011 | 0b010 => Instruction {
                opc: Arm32(Arm32::decode_single_data_transfer(instruction)),
                data: instruction,
                cond,
            },
            p @ 0b000 | p @ 0b001 => {
                if (p == 0b000
                    && ((instruction.bit(4) && !instruction.bit(7)) || !instruction.bit(4)))
                    || p == 0b001
                {
                    return Instruction {
                        opc: Arm32(Arm32::decode_data_processing_psr_transfer(instruction)),
                        data: instruction,
                        cond,
                    };
                }
                if instruction.bit_range(4..8) == 0b1001 {
                    return match instruction.bit_range(23..25) {
                        0 => Instruction {
                            opc: Arm32(Arm32::decode_mul(instruction)),
                            data: instruction,
                            cond,
                        },
                        0b01 => Instruction {
                            opc: Arm32(Arm32::decode_mul_long(instruction)),
                            data: instruction,
                            cond,
                        },
                        0b10 => Instruction {
                            opc: Arm32(SWP),
                            data: instruction,
                            cond,
                        },
                        _ => Instruction {
                            opc: Arm32(UNDEF),
                            data: instruction,
                            cond,
                        },
                    };
                } else if instruction.bit(4)
                    && instruction.bit(7)
                    && instruction.bit_range(8..=11) == 0
                {
                    return Instruction {
                        opc: Arm32(Arm32::decode_hdt(instruction)),
                        data: instruction,
                        cond,
                    };
                }
                return Instruction {
                    opc: Arm32(UNDEF),
                    data: instruction,
                    cond,
                };
            }
            _ => Instruction {
                opc: Arm32(UNDEF),
                data: instruction,
                cond,
            },
        }
    }
}
