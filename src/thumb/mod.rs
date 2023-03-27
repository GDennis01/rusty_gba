pub mod isa;
use crate::cpu::{
    Condition::*,
    Instruction,
    Opcode::{self, *},
};
use crate::BitRange;
use isa::OpcodeThumb::*;

pub struct Thumb {}
impl Thumb {
    pub fn decode(instruction: u32) -> Instruction {
        let mut opc = Thumb(UNDEF);
        let cond = AL;
        let data = instruction;

        if instruction.bit_range(8..=15) == 0b1101111 {
            opc = Thumb(SWI)
        }
        match instruction.bit_range(13..=15) {
            0b000 => {
                if instruction.bit_range(11..=12) == 0b11 {
                    opc = Thumb::decode_move_shifting_register(instruction)
                } else {
                    opc = Thumb::decode_add_sub(instruction)
                }
            }

            0b001 => opc = Thumb::decode_mcas_imm(instruction),
            0b010 => match instruction.bit_range(10..=12) {
                0b000 => opc = Thumb::decode_alu_op(instruction),
                0b001 => opc = Thumb::decode_hireg_bx(instruction),
                0b010 | 0b011 => opc = Thumb::decode_pc_relative_load(instruction),
                0b100 | 0b101 | 0b110 | 0b111 if !instruction.bit(9) => {
                    opc = Thumb::decode_loadstore_offset(instruction)
                }
                0b100 | 0b101 | 0b110 | 0b111 => {
                    opc = Thumb::decode_loadstore_signext_bytehalfword(instruction)
                }
                _ => opc = Thumb(UNDEF),
            },
            0b011 => opc = Thumb::decode_loadstore_imm(instruction),
            0b100 if !instruction.bit(12) => opc = Thumb::decode_loadstore_halfword(instruction),
            0b100 => opc = Thumb::decode_sprelative_loadstore(instruction),
            0b101 if instruction.bit(12) => opc = Thumb::decode_load_address(instruction),
            0b101 => match instruction.bit_range(8..=12) {
                0b10000 => opc = Thumb::decode_addoffset_sp(instruction),
                0b10100 | 0b10101 | 0b11100 | 0b11101 => opc = Thumb::decode_push_pop(instruction),
                _ => opc = Thumb(UNDEF),
            },

            0b110 => {
                if instruction.bit(12) {
                    opc = Thumb(Bxx);
                } else {
                    opc = Thumb::decode_multiple_loadstore(instruction);
                }
            }
            0b111 => {
                if instruction.bit(12) {
                    opc = Thumb(B);
                } else {
                    opc = Thumb(BL);
                }
            }
            _ => opc = Thumb(UNDEF),
        };
        Instruction { opc, data, cond }
    }

    fn decode_move_shifting_register(instruction: u32) -> Opcode {
        Thumb(LSL)
    }

    fn decode_add_sub(instruction: u32) -> Opcode {
        Thumb(ADD)
    }
    fn decode_mcas_imm(instruction: u32) -> Opcode {
        Thumb(ADD)
    }
    fn decode_alu_op(instruction: u32) -> Opcode {
        Thumb(ADD)
    }
    fn decode_loadstore_imm(instruction: u32) -> Opcode {
        Thumb(ADD)
    }
    fn decode_multiple_loadstore(instruction: u32) -> Opcode {
        Thumb(ADD)
    }
    fn decode_hireg_bx(instruction: u32) -> Opcode {
        Thumb(ADD)
    }
    fn decode_pc_relative_load(instruction: u32) -> Opcode {
        Thumb(ADD)
    }
    fn decode_loadstore_offset(instruction: u32) -> Opcode {
        Thumb(ADD)
    }
    fn decode_loadstore_signext_bytehalfword(instruction: u32) -> Opcode {
        Thumb(ADD)
    }
    fn decode_loadstore_halfword(instruction: u32) -> Opcode {
        Thumb(ADD)
    }
    fn decode_sprelative_loadstore(instruction: u32) -> Opcode {
        Thumb(ADD)
    }
    fn decode_load_address(instruction: u32) -> Opcode {
        Thumb(ADD)
    }
    fn decode_addoffset_sp(instruction: u32) -> Opcode {
        Thumb(ADD)
    }
    fn decode_push_pop(instruction: u32) -> Opcode {
        Thumb(ADD)
    }
}
