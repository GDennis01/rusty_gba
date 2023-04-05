use crate::{
    arm32::{
        isa::OpcodeArm::{self, *},
        Arm32,
    },
    thumb::{isa::OpcodeThumb, Thumb},
    BitRange,
};
use std::{fmt, ops::Index};

//Used to index Special PSR
const r8_fiq: u8 = 17;
const r9_fiq: u8 = 18;
const r10_fiq: u8 = 19;
const r11_fiq: u8 = 20;
const r12_fiq: u8 = 21;
const r13_fiq: u8 = 22;
const r14_fiq: u8 = 23;
const r13_irq: u8 = 24;
const r14_irq: u8 = 25;
const r13_svc: u8 = 26;
const r14_svc: u8 = 27;
const r13_abt: u8 = 28;
const r14_abt: u8 = 29;
const r13_und: u8 = 30;
const r14_und: u8 = 31;

///Arm7tdmi's CPU<br>
///Has 2 CPU operating modes: 32 bit(Arm) and 16 bit(Thumb)<br>
///Has 6 "user" modes, each with its own privileges, used for various Interrupts
pub struct CPU<T: MemoryInterface + Default> {
    registers: [u32; 31],
    //User/Sys, FIQ, IRQ, Supervisor, Abort, Undefined
    pub psr: [PSR; 6],
    pipeline: [u32; 3],
    pub mode: Mode,
    pub operating_mode: OperatingMode,
    pub memory: Box<T>,
}
impl<T: MemoryInterface + Default> CPU<T> {
    pub fn new() -> Self {
        CPU {
            registers: [0; 31],
            psr: [PSR::new(); 6],
            pipeline: [0; 3],
            mode: Mode::ARM,
            operating_mode: OperatingMode::User,
            memory: Box::new(T::default()),
        }
    }
    ///Based on the current CPU operating mode, decodes an instruction in Arm or Thumb mode
    pub fn decode(&self, instruction: u32) -> Instruction {
        match &self.mode {
            Mode::ARM => Arm32::decode(instruction),
            Mode::THUMB => Thumb::decode(instruction),
        }
    }

    ///Evaluate a condition based on PSR flags
    pub fn evaluate_cond(&self, cond: Condition) -> bool {
        let op_mode = self.operating_mode;
        let _psr = self.psr[op_mode];
        match cond {
            Condition::EQ => _psr.get_z(),
            Condition::NE => !_psr.get_z(),
            Condition::CS => _psr.get_c(),
            Condition::CC => !_psr.get_c(),
            Condition::MI => _psr.get_n(),
            Condition::PL => !_psr.get_n(),
            Condition::VS => _psr.get_v(),
            Condition::VC => !_psr.get_v(),
            Condition::HI => _psr.get_c() && !_psr.get_z(),
            Condition::LS => !_psr.get_c() || _psr.get_z(),
            Condition::GE => _psr.get_n() == _psr.get_v(),
            Condition::LT => _psr.get_n() != _psr.get_v(),
            Condition::GT => !_psr.get_z() && (_psr.get_n() == _psr.get_v()),
            Condition::LE => _psr.get_z() || (_psr.get_n() != _psr.get_v()),
            Condition::AL => true,
            Condition::ERR => false,
        }
    }

    //TODO: Using fx pointers?
    ///Execute an arm instruction based on its opcode
    pub fn execute_arm(instruction: Instruction) {
        match instruction.opc {
            Opcode::Arm32(ADC) => todo!(),
            Opcode::Arm32(ADD) => todo!(),
            Opcode::Arm32(AND) => todo!(),
            Opcode::Arm32(B) => todo!(),
            Opcode::Arm32(BIC) => todo!(),
            Opcode::Arm32(BX) => todo!(),
            Opcode::Arm32(CMN) => todo!(),
            Opcode::Arm32(CMP) => todo!(),
            Opcode::Arm32(EOR) => todo!(),
            Opcode::Arm32(LDM) => todo!(),
            Opcode::Arm32(LDR) => todo!(),
            Opcode::Arm32(LDRB) => todo!(),
            Opcode::Arm32(LDRH) => todo!(),
            Opcode::Arm32(LDRSB) => todo!(),
            Opcode::Arm32(LDRSH) => todo!(),
            Opcode::Arm32(MLA) => todo!(),
            Opcode::Arm32(MOV) => todo!(),
            Opcode::Arm32(MRS) => todo!(),
            Opcode::Arm32(MSR) => todo!(),
            Opcode::Arm32(MUL) => todo!(),
            Opcode::Arm32(MVN) => todo!(),
            Opcode::Arm32(ORR) => todo!(),
            Opcode::Arm32(RSB) => todo!(),
            Opcode::Arm32(RSC) => todo!(),
            Opcode::Arm32(SBC) => todo!(),
            Opcode::Arm32(SMLAL) => todo!(),
            Opcode::Arm32(SMULL) => todo!(),
            Opcode::Arm32(STM) => todo!(),
            Opcode::Arm32(STR) => todo!(),
            Opcode::Arm32(STRB) => todo!(),
            Opcode::Arm32(STRH) => todo!(),
            Opcode::Arm32(SUB) => todo!(),
            Opcode::Arm32(SWI) => todo!(),
            Opcode::Arm32(SWP) => todo!(),
            Opcode::Arm32(SWPB) => todo!(),
            Opcode::Arm32(TEQ) => todo!(),
            Opcode::Arm32(TST) => todo!(),
            Opcode::Arm32(UMLAL) => todo!(),
            Opcode::Arm32(UMULL) => todo!(),
            Opcode::Arm32(UNDEF) => todo!(),
            _ => todo!(),
        }
    }
}

///Program Status Register, a special register containing various flag of current CPU state
#[derive(Default, Copy, Clone)]
pub struct PSR {
    register: u32,
}

impl PSR {
    pub fn new() -> Self {
        PSR { register: 0 }
    }
    //Getters
    #[inline(always)]
    pub fn get_n(&self) -> bool {
        self.register.bit(31)
    }
    #[inline(always)]
    pub fn get_z(&self) -> bool {
        self.register.bit(30)
    }
    #[inline(always)]
    pub fn get_c(&self) -> bool {
        self.register.bit(29)
    }
    #[inline(always)]
    pub fn get_v(&self) -> bool {
        self.register.bit(28)
    }
    #[inline(always)]
    pub fn get_t(&self) -> bool {
        self.register.bit(5)
    }

    //Setters
    #[inline(always)]
    pub fn set_n(&mut self, value: bool) {
        self.register = self.register.bit_range(31..=31) | (value as u32) << 31;
    }
    #[inline(always)]
    pub fn set_z(&mut self, value: bool) {
        self.register = self.register.bit_range(30..=30) | (value as u32) << 30;
    }
    #[inline(always)]
    pub fn set_c(&mut self, value: bool) {
        self.register = self.register.bit_range(29..=29) | (value as u32) << 29;
    }
    #[inline(always)]
    pub fn set_v(&mut self, value: bool) {
        self.register = self.register.bit_range(28..=28) | (value as u32) << 28;
    }
    #[inline(always)]
    pub fn set_t(&mut self, value: bool) {
        self.register = self.register.bit_range(5..=5) | (value as u32) << 5;
    }

    ///Returns the current user operating mode
    pub fn get_op_mode(&self) -> OperatingMode {
        match self.register.bit_range(4..=0) {
            0b10000 => OperatingMode::User,
            0b10001 => OperatingMode::FIQ,
            0b10010 => OperatingMode::IRQ,
            0b10011 => OperatingMode::Supervisor,
            0b10111 => OperatingMode::Abort,
            0b11011 => OperatingMode::Undefined,
            0b11111 => OperatingMode::System,
            _ => panic!("Invalid operating mode"),
        }
    }
}
impl Index<OperatingMode> for [PSR; 6] {
    type Output = PSR;
    fn index(&self, index: OperatingMode) -> &Self::Output {
        match index {
            OperatingMode::User | OperatingMode::System => &self[0],
            OperatingMode::FIQ => &self[1],
            OperatingMode::IRQ => &self[2],
            OperatingMode::Supervisor => &self[3],
            OperatingMode::Abort => &self[4],
            OperatingMode::Undefined => &self[5],
        }
    }
}
///Current Operating Mode. Usually starts in User mode.<br>
///Each operating mode has its own copy of Program Status Register (PSR for short)
#[derive(Copy, Clone, Debug)]
pub enum OperatingMode {
    User = 0b10000,
    FIQ = 0b10001,
    IRQ = 0b10010,
    Supervisor = 0b10011,
    Abort = 0b10111,
    Undefined = 0b11011,
    System = 0b11111,
}

///Simple trait with methods to read/write 8bit,16 bit or 32 bit.<br>
///Implementors should use a wrapping structur of an u8 array as underlying data structure that implements this trait
pub trait MemoryInterface {
    fn new() -> Self;
    fn read_8(&self, address: u32) -> u8;
    fn read_16(&self, address: u32) -> u16;
    fn read_32(&self, address: u32) -> u32;
    fn write_8(&mut self, address: u32, value: u8);
    fn write_16(&mut self, address: u32, value: u16);
    fn write_32(&mut self, address: u32, value: u32);
}

///Enum that contains both ARM and Thumb Opcodes
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

pub struct Instruction {
    pub opc: Opcode,
    pub data: u32,
    pub cond: Condition,
    // pub fx: FxArm32,
}
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.cond {
            Condition::AL => write!(f, "{:?} {:#06x?}", self.opc, self.data),
            _ => write!(f, "{:?}{:?} {:#06x?}", self.opc, self.cond, self.data),
        }
    }
}

///Enum that contains CPU operating modes: Arm(32 bit) or Thumb(16 bit)
#[derive(Copy, Clone)]
pub enum Mode {
    ARM,
    THUMB,
}

///Enum that contains instruction conditions(4 uppermost bits) on Arm instructions
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
