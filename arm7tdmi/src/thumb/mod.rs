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
    //TODO: Testare il decode
    pub fn decode(instruction: u32) -> Instruction {
        let mut opc = Thumb(UNDEF);
        let cond = AL;
        let data = instruction;

        if instruction.bit_range(8..=15) == 0b1101111 {
            opc = Thumb(SWI) //Format17: Software Interrupt
        }
        match instruction.bit_range(13..=15) {
            0b000 => {
                if instruction.bit_range(11..=12) != 0b11 {
                    opc = Thumb::decode_move_shifting_register(instruction)
                } else {
                    opc = Thumb::decode_add_sub(instruction)
                }
            }

            0b001 => opc = Thumb::decode_mcas_imm(instruction),
            0b010 => match instruction.bit_range(10..=12) {
                0b000 => opc = Thumb::decode_alu_op(instruction),
                0b001 => opc = Thumb::decode_hireg_bx(instruction),
                //Format6: Adds an u-offset to pc and store the result in rd
                0b010 | 0b011 => opc = Thumb(LDR), //LDR1
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
            0b101 if instruction.bit(12) => opc = Thumb::decode_load_address(),
            0b101 => match instruction.bit_range(8..=12) {
                0b10000 => opc = Thumb::decode_addoffset_sp(),
                0b10100 | 0b10101 | 0b11100 | 0b11101 => opc = Thumb::decode_push_pop(instruction),
                _ => opc = Thumb(UNDEF),
            },

            0b110 => {
                if instruction.bit(12) {
                    opc = Thumb(Bxx); //Format16: Conditional branch
                } else {
                    opc = Thumb::decode_multiple_loadstore(instruction);
                }
            }
            0b111 => {
                if !instruction.bit(12) && !instruction.bit(11) {
                    opc = Thumb(B); //Format18: Unconditional Branch
                } else if instruction.bit(12) {
                    opc = Thumb(BL); //Format19: Long branch with Link
                } else {
                    opc = Thumb(UNDEF);
                }
            }
            _ => opc = Thumb(UNDEF),
        };
        Instruction { opc, data, cond }
    }

    //Format1: Perform shifting on a Lo reg
    fn decode_move_shifting_register(instruction: u32) -> Opcode {
        match instruction.bit_range(11..=12) {
            0 => Thumb(LSL), //LSL1
            1 => Thumb(LSR), //LSRL1
            2 => Thumb(ASR), //ASR1
            _ => Thumb(UNDEF),
        }
    }

    //Format2: ADD/SUB with Lo reg only and(opt) immediate(3bit) value!
    fn decode_add_sub(instruction: u32) -> Opcode {
        if instruction.bit(9) {
            Thumb(SUB) //SUB1
        } else {
            Thumb(ADD) //ADD1
        }
    }

    //Format3: Mov/Cmp/Add/Sub with Lo reg and immediate(8bit) value
    fn decode_mcas_imm(instruction: u32) -> Opcode {
        match instruction.bit_range(11..=12) {
            0 => Thumb(MOV), //MOV1
            1 => Thumb(CMP), //CMP1
            2 => Thumb(ADD), //ADD2
            3 => Thumb(SUB), //SUB2
            _ => Thumb(UNDEF),
        }
    }
    //Format4: ALU Operations between Lo reg pair
    fn decode_alu_op(instruction: u32) -> Opcode {
        match instruction.bit_range(6..=9) {
            0b0000 => Thumb(AND),
            0b0001 => Thumb(EOR),
            0b0010 => Thumb(LSL), //LSL2
            0b0011 => Thumb(LSR), //LSR2
            0b0100 => Thumb(ASR), //ASR2
            0b0101 => Thumb(ADC),
            0b0110 => Thumb(SBC),
            0b0111 => Thumb(ROR),
            0b1000 => Thumb(TST),
            0b1001 => Thumb(NEG),
            0b1010 => Thumb(CMP), //CMP2
            0b1011 => Thumb(CMN),
            0b1100 => Thumb(ORR),
            0b1101 => Thumb(MUL),
            0b1110 => Thumb(BIC),
            0b1111 => Thumb(MVN),
            _ => Thumb(UNDEF),
        }
    }
    //Format5: Operations between Lo-Hi, Hi-Lo or Hi-Hi reg pairs. Pairs type are indicated by bit(7) and bit(6) flags(H2 and H1)
    fn decode_hireg_bx(instruction: u32) -> Opcode {
        match instruction.bit_range(8..=9) {
            0b00 => Thumb(ADD), //ADD3
            0b01 => Thumb(CMP), //CMP3
            0b10 => Thumb(MOV), //MOV2
            0b11 => Thumb(BX),
            _ => Thumb(UNDEF),
        }
    }
    //Format7: Word/Byte transfer between Lo registers and Memory
    fn decode_loadstore_offset(instruction: u32) -> Opcode {
        match instruction.bit_range(10..=11) {
            //bit(11) is the Load/Store flag, bit(10) is the Word/Byte flag
            0b00 => Thumb(STR),  //STR1
            0b01 => Thumb(STRB), //STRB1
            0b10 => Thumb(LDR),  //LDR2
            0b11 => Thumb(LDRB), //LDRB1
            _ => Thumb(UNDEF),
        }
    }
    //Format8: Load (sign-extended)byte/halfword and store halfwords
    fn decode_loadstore_signext_bytehalfword(instruction: u32) -> Opcode {
        match instruction.bit_range(10..=11) {
            //bit(11) is the Load/Store flag, bit(10) is the Word/Byte flag
            0b00 => Thumb(STRH), //STRH1
            0b01 => Thumb(LDSB),
            0b10 => Thumb(LDRH), //LDRH1
            0b11 => Thumb(LDSH),
            _ => Thumb(UNDEF),
        }
    }
    //Format9: Byte/word transfer between registers using immediate 5/7 bit offset
    fn decode_loadstore_imm(instruction: u32) -> Opcode {
        match instruction.bit_range(11..=12) {
            0b00 => Thumb(STR),  //STR2
            0b01 => Thumb(LDR),  //LDR3
            0b10 => Thumb(STRB), //STRB2
            0b11 => Thumb(LDRB), //LDRB2
            _ => Thumb(UNDEF),
        }
    }
    //Format10: Halfword transfer between Lo reg and memory
    fn decode_loadstore_halfword(instruction: u32) -> Opcode {
        if !instruction.bit(11) {
            Thumb(STRH) //STRH2
        } else {
            Thumb(LDRH) //LDRH2
        }
    }
    //Format11: SP-relative load stor
    fn decode_sprelative_loadstore(instruction: u32) -> Opcode {
        if !instruction.bit(11) {
            Thumb(STR) //STR3
        } else {
            Thumb(LDR) //LDR4
        }
    }
    //Format12: Adds an 8bit immediate value to either PC or SP. PC/SP indicated by bit 11
    fn decode_load_address() -> Opcode {
        Thumb(ADD) //ADD4
    }
    //Format13: Adds a 9bit signed constant to SP. Sign indicated by bit 7
    fn decode_addoffset_sp() -> Opcode {
        Thumb(ADD) //ADD5
    }
    //Format14: Push Lo reg(and LR) into stack or Pop Lo reg(and PC) from stack based on bit 8(for LR and PC).
    fn decode_push_pop(instruction: u32) -> Opcode {
        if !instruction.bit(11) {
            Thumb(PUSH)
        } else {
            Thumb(POP)
        }
    }
    //Format15: Multiple loading and storing of Lo reg
    fn decode_multiple_loadstore(instruction: u32) -> Opcode {
        if !instruction.bit(11) {
            Thumb(STMIA)
        } else {
            Thumb(LDMIA)
        }
    }
}
