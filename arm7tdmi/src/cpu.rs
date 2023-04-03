use crate::{
    arm32::{isa::OpcodeArm, Arm32},
    thumb::{isa::OpcodeThumb, Thumb},
    BitRange,
};
use std::fmt;

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

///CPU of Arm7tdmi
pub struct CPU<T: MemoryInterface + Default> {
    registers: [u32; 31],
    //User/Sys, FIQ, IRQ, Supervisor, Abort, Undefined
    psr: [PSR; 6],
    pipeline: [u32; 3],
    mode: Mode,
    operating_mode: OperatingMode,
    pub memory: T,
}
impl<T: MemoryInterface + Default> CPU<T> {
    pub fn new() -> Self {
        CPU {
            registers: [0; 31],
            psr: [PSR::new(); 6],
            pipeline: [0; 3],
            mode: Mode::ARM,
            operating_mode: OperatingMode::User,
            memory: T::default(),
        }
    }
    ///Based on the current operating mode, executes an instruction in Arm or Thumb mode
    pub fn decode(&self, instruction: u32) -> Instruction {
        match &self.mode {
            Mode::ARM => Arm32::decode(instruction),
            Mode::THUMB => Thumb::decode(instruction),
        }
    }
    ///Returns the current CPU operating mode(Thumb or ARM)
    pub fn get_mode(&self) -> Mode {
        self.mode
    }
}
///Program Status Register
#[derive(Default, Copy, Clone)]
pub struct PSR {
    register: u32,
}
//Getter/Setter methods
impl PSR {
    pub fn new() -> Self {
        PSR { register: 0 }
    }
    //Getters
    pub fn get_n(&self) -> bool {
        self.register.bit(31)
    }
    pub fn get_z(&self) -> bool {
        self.register.bit(30)
    }
    pub fn get_c(&self) -> bool {
        self.register.bit(29)
    }
    pub fn get_v(&self) -> bool {
        self.register.bit(28)
    }
    pub fn get_t(&self) -> bool {
        self.register.bit(5)
    }
    //Setters
    pub fn set_n(&mut self, value: bool) {
        self.register = self.register.bit_range(31..=31) | (value as u32) << 31;
    }
    pub fn set_z(&mut self, value: bool) {
        self.register = self.register.bit_range(30..=30) | (value as u32) << 30;
    }
    pub fn set_c(&mut self, value: bool) {
        self.register = self.register.bit_range(29..=29) | (value as u32) << 29;
    }
    pub fn set_v(&mut self, value: bool) {
        self.register = self.register.bit_range(28..=28) | (value as u32) << 28;
    }
    pub fn set_t(&mut self, value: bool) {
        self.register = self.register.bit_range(5..=5) | (value as u32) << 5;
    }

    ///Returns the current operating mode
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
///Current Operating Mode. Usually starts in User mode.<br>
///User and System shares same registers.
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
///Implementors should use an u8 array as underlying data structure that implements this trait
pub trait MemoryInterface {
    fn new() -> Self;
    fn read_8(&self, address: u32) -> u8;
    fn read_16(&self, address: u32) -> u16;
    fn read_32(&self, address: u32) -> u32;
    fn write_8(&mut self, address: u32, value: u8);
    fn write_16(&mut self, address: u32, value: u16);
    fn write_32(&mut self, address: u32, value: u32);
}
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
#[derive(Copy, Clone)]
pub enum Mode {
    ARM,
    THUMB,
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
