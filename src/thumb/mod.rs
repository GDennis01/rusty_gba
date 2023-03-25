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
            opc = Thumb(SWI);
        }
        match instruction.bit_range(13..=15) {
            0b000 => {
                if instruction.bit_range(11..=12) == 0b11 {
                    opc = Thumb::decode_move_shifting_register(instruction);
                } else {
                    opc = Thumb::decode_add_sub(instruction);
                }
            }

            0b001 => opc = Thumb::decode_mcas_imm(instruction),
            0b010 => opc = Thumb::decode_alu_op(instruction),
            0b011 => opc = Thumb::decode_loadstore_imm(instruction),
            0b100 => {}
            0b101 => {}
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
}
