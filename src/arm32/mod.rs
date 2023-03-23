//TODO: Implementare ciclo fetch decode execute
//FIXME: 0000 0000 0000 0000 0010 0110 1001 0011 (0x2693) MUL
pub mod isa;
// use bit::BitIndex;
use isa::Condition::{self, *};
use isa::Instruction;
use isa::Opcode::{self, *};
use std::ops::RangeBounds;
use std::ops::{Range, RangeInclusive};

// const OPC_1: u8 = 0xF0; //bits[4..=7]
const OPC_1: Range<usize> = 4..8; //bits[4..=7]                        // const OPC_2: u8 = 0xFF00000; //bits[20..=27]
const OPC_2: Range<usize> = 20..28; //bits[20..=27]
                                    // const _COND: u8 = 0xF0000000; //bits[28..=31]
const COND: Range<usize> = 28..32; //bits[28..=31]

struct Registers {
    registers: [u32; 15],
    spr: u32,
}
//FIXME: organize it better
pub struct Arm32 {
    raw_ins: u32,
    pipeline: [u32; 3],
    _registers: Registers,
}
/// Returns the number in the given range
/// 111011.bit_range(2..4) would return 101
pub trait BitRange {
    fn bit_range<R: RangeBounds<u8>>(&self, range: R) -> Self;
    fn bit(&self, bit: u8) -> bool;
}

impl BitRange for u32 {
    fn bit_range<R: RangeBounds<u8>>(&self, range: R) -> Self {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&n) => n,
            std::ops::Bound::Excluded(&n) => n - 1,
            std::ops::Bound::Unbounded => 0,
        };
        let end: u8 = match range.end_bound() {
            std::ops::Bound::Included(&n) => n,
            std::ops::Bound::Excluded(&n) => n - 1,
            std::ops::Bound::Unbounded => 0,
        };
        (self << (31 - end)) >> (31 - (end - start)) //still gotta test
    }
    fn bit(&self, bit: u8) -> bool {
        (self << (31 - bit)) >> (31 - (bit - bit)) == 1
    }
}
impl Arm32 {
    pub fn new() -> Arm32 {
        Arm32 {
            raw_ins: 0,
            pipeline: [0, 0, 0],
            _registers: Registers {
                registers: [0; 15],
                spr: 0,
            },
        }
    }
    fn decode_block_data_transfer(&self, instruction: u32) -> Opcode {
        match instruction.bit(20) {
            true => LDM,
            false => STM,
        }
    }
    fn decode_single_data_transfer(&self, instruction: u32) -> Opcode {
        match instruction.bit(20) {
            true => LDR,
            false => STR,
        }
    }
    fn decode_psr(&self, instruction: u32, opc: Opcode) -> Opcode {
        match opc {
            MRS => {
                if instruction.bit_range(16..22) == 0b001111 && instruction.bit_range(0..12) == 0 {
                    MRS
                } else {
                    UNDEF
                }
            }
            MSR => {
                if instruction.bit_range(12..22) == 0b1010011111 {
                    MSR
                } else {
                    UNDEF
                }
            }
            _ => UNDEF,
        }
    }
    fn decode_data_processing_psr_transfer(&self, instruction: u32) -> Opcode {
        match instruction.bit_range(21..25) {
            0b0000 => AND,
            0b0001 => EOR,
            0b0010 => SUB,
            0b0011 => RSB,
            0b0100 => ADD,
            0b0101 => ADC,
            0b0110 => SBC,
            0b0111 => RSC,
            0b1000 => {
                return if !instruction.bit(20) {
                    self.decode_psr(instruction, MRS)
                } else {
                    TST
                }
            }
            0b1001 => {
                return if !instruction.bit(20) {
                    self.decode_psr(instruction, MSR)
                } else {
                    TEQ
                }
            }
            0b1010 => {
                return if !instruction.bit(20) {
                    self.decode_psr(instruction, MRS)
                } else {
                    CMP
                }
            }
            0b1011 => {
                return if !instruction.bit(20) {
                    self.decode_psr(instruction, MSR)
                } else {
                    CMN
                }
            }
            0b1100 => ORR,
            0b1101 => MOV,
            0b1110 => BIC,
            0b1111 => MVN,
            _ => UNDEF,
        }
    }
    fn decode_mul(&self, instruction: u32) -> Opcode {
        match instruction.bit(21) {
            true => MLA,
            false => MUL,
        }
    }
    fn decode_mul_long(&self, instruction: u32) -> Opcode {
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
    fn decode_hdt(&self, instruction: u32) -> Opcode {
        let r = instruction.bit_range(5..7);
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
    fn get_condition(&self, instruction: u32) -> Condition {
        return match instruction.bit_range(28..32) {
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
    pub fn decode(&self, instruction: u32) -> Instruction {
        let cond: Condition = self.get_condition(instruction);
        if instruction.bit_range(4..28) == 0x12FFF1 {
            return Instruction {
                opc: BX,
                data: instruction,
                cond,
            };
        }
        if instruction.bit_range(24..28) == 0b1111 {
            return Instruction {
                opc: SWI,
                data: instruction,
                cond,
            };
        }
        // if instruction.bit_range(25..28) == 0b011 && instruction.bit(4) {
        //     return Instruction {
        //         opc: UNDEF,
        //         data: instruction,
        //         cond,
        //     };
        // }
        return match instruction.bit_range(25..28) {
            0b011 if instruction.bit(4) => Instruction {
                opc: UNDEF,
                data: instruction,
                cond,
            },
            0b101 => Instruction {
                opc: B,
                data: instruction,
                cond,
            },
            0b100 => Instruction {
                opc: self.decode_block_data_transfer(instruction),
                data: instruction,
                cond,
            },
            0b011 | 0b010 => Instruction {
                opc: self.decode_single_data_transfer(instruction),
                data: instruction,
                cond,
            },
            p @ 0b000 | p @ 0b001 => {
                if (p == 0b000
                    && ((instruction.bit(4) && !instruction.bit(7)) || !instruction.bit(4)))
                    || p == 0b001
                {
                    return Instruction {
                        opc: self.decode_data_processing_psr_transfer(instruction),
                        data: instruction,
                        cond,
                    };
                }
                if instruction.bit_range(4..8) == 0b1001 {
                    return match instruction.bit_range(23..25) {
                        0 => Instruction {
                            opc: self.decode_mul(instruction),
                            data: instruction,
                            cond,
                        },
                        0b01 => Instruction {
                            opc: self.decode_mul_long(instruction),
                            data: instruction,
                            cond,
                        },
                        0b10 => Instruction {
                            opc: SWP,
                            data: instruction,
                            cond,
                        },
                        _ => Instruction {
                            opc: UNDEF,
                            data: instruction,
                            cond,
                        },
                    };
                } else if instruction.bit(4)
                    && instruction.bit(7)
                    && instruction.bit_range(8..12) == 0
                {
                    return Instruction {
                        opc: self.decode_hdt(instruction),
                        data: instruction,
                        cond,
                    };
                }
                return Instruction {
                    opc: UNDEF,
                    data: instruction,
                    cond,
                };
            }
            _ => Instruction {
                opc: UNDEF,
                data: instruction,
                cond,
            },
        };
    }
}
