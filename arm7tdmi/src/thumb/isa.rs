use crate::cpu::{MemoryInterface, CPU};

// TODO: magari aggiungere le varie varianti di ADD,STR,LDR etc
#[derive(Debug)]
pub enum OpcodeThumb {
    ADC,
    ADD,
    AND,
    ASR,
    B,
    Bxx,
    BIC,
    BL,
    BX,
    CMN,
    CMP,
    EOR,
    LDMIA,
    LDR,
    LDRB,
    LDRH,
    LSL,
    LDSB,
    LDSH,
    LDRSH,
    LSR,
    MOV,
    MUL,
    MVN,
    NEG,
    ORR,
    POP,
    PUSH,
    ROR,
    SBC,
    STMIA,
    STR,
    STRB,
    STRH,
    SWI,
    SUB,
    TST,
    UNDEF,
}

impl<T: MemoryInterface + Default> CPU<T> {
    //ARM Thumb definitions
}
