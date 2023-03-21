pub mod isa;
use bit::BitIndex;
use isa::Opcode::{self, *};
use std::ops::Range;
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
    fn is_dp_psr(&self, instruction: u32) -> bool {
        if instruction.bit_range(OPC_2) & 0x20 == 0x20
            || instruction.bit_range(OPC_1) & 0x01 == 0x00
            || instruction.bit_range(OPC_1) == 0x00
        {
            true
        } else {
            false
        }
    }
    pub fn decode(&self, instruction: u32) -> Opcode {
        // Undef when [25:27] -> 011 && [4] set
        // if ((instruction & 0x0E00_0010) == 0x0600_0010) {
        //     return UNDEF;
        // }
        if instruction.bit_range(25..28) == 0b011 && instruction.bit(4) {
            return UNDEF;
        }

        //opcode for data processing/psr transfer
        // let opcode = (instruction_ins & COND) >> 28;
        let opcode = instruction.bit_range(COND);
        //i want to check only bits [26-27]
        match instruction.bit_range(26..28) {
            0x0 => {
                if self.is_dp_psr(instruction) {
                    match opcode {
                        0x0 => return AND,
                        0x1 => return EOR,
                        0x2 => return SUB,
                        0x3 => return RSB,
                        0x4 => return ADD,
                        0x5 => return ADC,
                        0x6 => return SBC,
                        0x7 => return RSC,
                        0x8 => {
                            if instruction.bit_range(OPC_2).bit(0) {
                                return TST;
                            }
                            // I[0] && SBO [16:19] && SBZ [0:11]
                            else if instruction.bit_range(0..12) == 0b0
                                && instruction.bit_range(16..22) == 0b001111
                                && instruction.bit_range(23..28) == 0b00010
                            {
                                return MRS;
                            } else {
                                return UNDEF;
                            }
                        }
                        0x9 => {
                            if instruction.bit(20) {
                                return TEQ;
                            //bit[16-19] and bit[20]
                            } else if instruction.bit(25) && instruction.bit_range(12..16) == 0b1111
                            {
                                return MSR;
                            } else if !instruction.bit(25)
                                && instruction.bit_range(11..16) == 0b11111
                                && instruction.bit_range(4..12) == 0
                            {
                                return MSR; //MSR immediate
                            } else if instruction.bit_range(4..28) == 0x000FFF10 {
                                return BX;
                            } else {
                                return UNDEF;
                            }
                        }
                        0xA => {
                            // if ((instruction & 0x0010F000) == 0x00100000) {
                            // S[1] && SBZ [12:15]
                            if instruction.bit(20) && instruction.bit_range(12..16) == 0 {
                                return CMP;
                            } else if instruction.bit_range(0..12) == 0b0
                                && instruction.bit_range(16..22) == 0b001111
                                && instruction.bit_range(23..28) == 0b00010
                            {
                                // I[0] && SBO [16:19] && SBZ [0:11]
                                return MRS;
                            } else {
                                return UNDEF;
                            }
                        }
                        0xB => {
                            if instruction.bit(20) && instruction.bit_range(12..16) == 0 {
                                // S[1] && SBZ [12:15]
                                return CMN;
                            } else if instruction.bit_range(12..16) == 0b1111 {
                                // I[1] && SBO [12:15]
                                return MSR;
                            } else if instruction.bit_range(4..16) == 0xF000 {
                                // I[0] && SBO [11:15] && SBZ [4:11]
                                return MSR;
                            } else {
                                return UNDEF;
                            }
                        }
                        0xC => return ORR,
                        0xD => {
                            if instruction.bit_range(16..20) == 0b0 {
                                // SBZ [16:19]
                                return MOV;
                            } else {
                                return UNDEF;
                            }
                        }
                        0xE => return BIC,
                        0xF => {
                            if instruction.bit_range(16..20) == 0b0 {
                                // SBZ [16:19]
                                return MVN;
                            } else {
                                return UNDEF;
                            }
                        }
                        _ => return UNDEF,
                    }
                }
                match instruction.bit_range(OPC_1) {
                    0x9 => match instruction.bit_range(OPC_2) {
                        0x0 | 0x1 => return MUL,
                        0x2 | 0x3 => return MLA,
                        0x8 | 0x9 => return UMULL,
                        0xA | 0xB => return UMLAL,
                        0xC | 0xD => return SMULL,
                        0xE | 0xF => return SMLAL,
                        0x10 => return SWP,
                        0x14 => return SWPB,
                        _ => return UNDEF,
                    },
                    0xB => {
                        if instruction.bit(20) {
                            return LDRH;
                        } else {
                            return STRH;
                        }
                    }
                    0xD => {
                        if instruction.bit(20) {
                            return LDRSB;
                        } else {
                            return UNDEF;
                        }
                    }
                    0xF => {
                        if instruction.bit(20) {
                            return LDRSH;
                        } else {
                            return UNDEF;
                        }
                    }
                    0xE | _ => return UNDEF,
                }
            }
            0x1 => {
                //isolate bits [20..=22]
                // match (((instruction & OPC_2) >> 20) & 0x5) {
                match instruction.bit_range(20..23) {
                    0x0 => return STR,
                    0x1 => return LDR,
                    0x4 => return STRB,
                    0x5 => return LDRB,
                    _ => return UNDEF,
                }
            }
            0x2 => {
                //isolate bits [20..=25]
                // match (((instruction & OPC_2) >> 20) & 0x21) {
                match instruction.bit_range(20..26) {
                    0x20 | 0x21 => B,
                    0x01 => LDM,
                    0x0 => STM,
                    _ => return UNDEF,
                }
            }
            0x3 => {
                if instruction.bit_range(24..28) == 0b1111 {
                    return SWI;
                } else {
                    return UNDEF;
                }
            }
            _ => return UNDEF,
        }

        // Check whether the instruction is a data processing or psr transfer one
    }
}
