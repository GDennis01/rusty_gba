use crate::{
    arm32::{
        isa::OpcodeArm::{self, *},
        Arm32,
    },
    cpu::OperatingMode::*,
    thumb::{isa::OpcodeThumb, Thumb},
    BitRange,
};
use alloc::boxed::Box;
use core::{fmt, ops::Index, ops::IndexMut};
// use std::{fmt, ops::Index, ops::IndexMut};

//Used to index  banked registers.
const FIQ_OFFSET: usize = 8; //from 16 to 22
const SVC_OFFSET: usize = 10; //23-24
const ABT_OFFSET: usize = 12; //25-26
const IRQ_OFFSET: usize = 14; //27-28
const UND_OFFSET: usize = 16; //29-30

///Arm7tdmi's CPU<br>
///Has 2 CPU operating modes: 32 bit(Arm) and 16 bit(Thumb)<br>
///Has 6 "user" modes, each with its own privileges, used for various Interrupts
pub struct CPU<T: MemoryInterface + Default> {
    pub registers: [u32; 31],
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
    /// Flush the pipeline.<br>
    /// Called whenever a write on R15(PC) occurs
    /// TODO
    pub fn flush_pipeline(&mut self) {}

    ///Get the specified  register value, taking into account banked registers
    /// # Arguments
    /// * **reg** - number of register to get from 0 to 15.
    pub fn get_register(&mut self, reg: u8) -> u32 {
        //only Hi registers(8-14) are banked
        if reg < 8 || reg == 15 {
            return self.registers[reg as usize];
        }
        match self.operating_mode {
            FIQ => self.registers[(reg as usize) + FIQ_OFFSET],
            IRQ if reg > 12 => self.registers[(reg as usize) + IRQ_OFFSET],
            Supervisor if reg > 12 => self.registers[(reg as usize) + SVC_OFFSET],
            Abort if reg > 12 => self.registers[(reg as usize) + ABT_OFFSET],
            Undefined if reg > 12 => self.registers[(reg as usize) + UND_OFFSET],
            User | System | _ => self.registers[reg as usize],
        }
    }

    /// Update current operating mode according to CPSR mode bits<br>
    /// Mostly called when a MSR occur
    /// * **is_special:** whether to use SPSR(true) or CPSR(false)
    ///
    pub fn update_operating_mode(&mut self, is_special: bool) {
        let index = if is_special {
            self.operating_mode as usize
        } else {
            0
        };
        match self.psr[index].register.bit_range(0..=4) {
            0b10000 => self.operating_mode = OperatingMode::User,
            0b10001 => self.operating_mode = OperatingMode::FIQ,
            0b10010 => self.operating_mode = OperatingMode::IRQ,
            0b10011 => self.operating_mode = OperatingMode::Supervisor,
            0b10111 => self.operating_mode = OperatingMode::Abort,
            0b11011 => self.operating_mode = OperatingMode::Undefined,
            0b11111 => self.operating_mode = OperatingMode::System,
            _ => self.operating_mode = OperatingMode::User,
        };
    }

    /// Set a value to the specified register, taking into account banked registers
    ///  # Arguments
    /// * **reg** - number of register to get from 0 to 15.
    /// * **data** - specified value to set
    pub fn set_register(&mut self, reg: u8, data: u32) {
        //only Hi registers(8-14) are banked
        if reg < 8 || reg == 15 {
            self.registers[reg as usize] = data;
            return;
        }
        match self.operating_mode {
            FIQ => self.registers[(reg as usize) + FIQ_OFFSET] = data,
            IRQ if reg > 12 => self.registers[(reg as usize) + IRQ_OFFSET] = data,
            Supervisor if reg > 12 => self.registers[(reg as usize) + SVC_OFFSET] = data,
            Abort if reg > 12 => self.registers[(reg as usize) + ABT_OFFSET] = data,
            Undefined if reg > 12 => self.registers[(reg as usize) + UND_OFFSET] = data,
            User | System | _ => self.registers[reg as usize] = data,
        }
    }
    //TODO: Using fx pointers?
    ///Execute an arm instruction based on its opcode
    pub fn execute_arm(&mut self, instruction: Instruction) {
        self.registers[15] += 4;
        if !self.evaluate_cond(instruction.cond) {
            return;
        }
        match instruction.opc {
            Opcode::Arm32(ADC) => self.ADC(instruction.data),
            Opcode::Arm32(ADD) => self.ADD(instruction.data),
            Opcode::Arm32(AND) => self.AND(instruction.data),
            Opcode::Arm32(B) => todo!(),
            Opcode::Arm32(BIC) => self.BIC(instruction.data),
            Opcode::Arm32(BX) => todo!(),
            Opcode::Arm32(CMN) => self.CMN(instruction.data),
            Opcode::Arm32(CMP) => self.CMP(instruction.data),
            Opcode::Arm32(EOR) => self.EOR(instruction.data),
            Opcode::Arm32(LDM) => todo!(),
            // Opcode::Arm32(LDR) => self.LDR(instruction.data),
            // Opcode::Arm32(LDRB) => self.LDR(instruction.data),
            Opcode::Arm32(LDR) => self.LDR_STR(instruction.data, OpcodeArm::LDR),
            Opcode::Arm32(LDRB) => self.LDR_STR(instruction.data, OpcodeArm::LDR),
            Opcode::Arm32(LDRH) => self.LDRH(instruction.data),
            Opcode::Arm32(LDRSB) => self.LDRSB(instruction.data),
            Opcode::Arm32(LDRSH) => self.LDRSH(instruction.data),
            Opcode::Arm32(MLA) => self.MLA(instruction.data),
            Opcode::Arm32(MOV) => self.MOV(instruction.data),
            Opcode::Arm32(MRS) => self.MRS(instruction.data),
            Opcode::Arm32(MSR) => self.MSR(instruction.data),
            Opcode::Arm32(MUL) => self.MUL(instruction.data),
            Opcode::Arm32(MVN) => self.MVN(instruction.data),
            Opcode::Arm32(ORR) => self.ORR(instruction.data),
            Opcode::Arm32(RSB) => self.RSB(instruction.data),
            Opcode::Arm32(RSC) => self.RSC(instruction.data),
            Opcode::Arm32(SBC) => self.SBC(instruction.data),
            Opcode::Arm32(SMLAL) => self.SMLAL(instruction.data),
            Opcode::Arm32(SMULL) => self.SMULL(instruction.data),
            Opcode::Arm32(STM) => todo!(),
            // Opcode::Arm32(STR) => self.STR(instruction.data),
            // Opcode::Arm32(STRB) => self.STR(instruction.data),
            Opcode::Arm32(STR) => self.LDR_STR(instruction.data, OpcodeArm::STR),
            Opcode::Arm32(STRB) => self.LDR_STR(instruction.data, OpcodeArm::STR),
            Opcode::Arm32(STRH) => todo!(),
            Opcode::Arm32(SUB) => self.SUB(instruction.data),
            Opcode::Arm32(SWI) => todo!(),
            Opcode::Arm32(SWP) => todo!(),
            Opcode::Arm32(SWPB) => todo!(),
            Opcode::Arm32(TEQ) => self.TEQ(instruction.data),
            Opcode::Arm32(TST) => self.TST(instruction.data),
            Opcode::Arm32(UMLAL) => self.UMLAL(instruction.data),
            Opcode::Arm32(UMULL) => self.UMULL(instruction.data),
            Opcode::Arm32(UNDEF) => todo!(),
            _ => todo!(),
        }
    }
}

///Program Status Register, a special register containing various flag of current CPU state
#[derive(Default, Copy, Clone)]
pub struct PSR {
    pub register: u32,
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
    /// Set N flag in the PSR
    pub fn set_n(&mut self, value: bool) {
        // self.register = self.register.bit_range(31..=31) | (value as u32) << 31;
        let data: u32 = if value { 0xFFFF_FFFF } else { 0 };
        self.register = self.register.set_bits(31..=31, data);
    }
    #[inline(always)]
    /// Set Z flag in the PSR
    pub fn set_z(&mut self, value: bool) {
        let data: u32 = if value { 0xFFFF_FFFF } else { 0 };
        // self.register = self.register.bit_range(30..=30) | (value as u32) << 30;
        self.register = self.register.set_bits(30..=30, data);
    }
    #[inline(always)]
    /// Set C flag in the PSR
    pub fn set_c(&mut self, value: bool) {
        // self.register = self.register.bit_range(29..=29) | (value as u32) << 29;
        let data: u32 = if value { 0xFFFF_FFFF } else { 0 };
        self.register = self.register.set_bits(29..=29, data);
    }
    #[inline(always)]
    /// Set V flag in the PSR
    pub fn set_v(&mut self, value: bool) {
        // self.register = self.register.bit_range(28..=28) | (value as u32) << 28;
        let data: u32 = if value { 0xFFFF_FFFF } else { 0 };
        self.register = self.register.set_bits(28..=28, data);
    }
    #[inline(always)]
    /// Set the T flag in the PSR
    pub fn set_t(&mut self, value: bool) {
        // self.register = self.register.bit_range(5..=5) | (value as u32) << 5;
        let data: u32 = if value { 0xFFFF_FFFF } else { 0 };
        self.register = self.register.set_bits(5..=5, data);
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
/// To index [PSR;6], alas an array of 6 PSR element.
/// This allows, e.g., to do psr[1] where psr is of type [PSR;6]
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
/// To index [PSR;6], alas an array of 6 PSR element.
/// This allows, e.g., to do psr[1] where psr is of type [PSR;6]
/// Used in a mutable context (e.g. psr[1]= bla bla)
impl IndexMut<OperatingMode> for [PSR; 6] {
    fn index_mut(&mut self, index: OperatingMode) -> &mut Self::Output {
        match index {
            OperatingMode::User | OperatingMode::System => &mut self[0],
            OperatingMode::FIQ => &mut self[1],
            OperatingMode::IRQ => &mut self[2],
            OperatingMode::Supervisor => &mut self[3],
            OperatingMode::Abort => &mut self[4],
            OperatingMode::Undefined => &mut self[5],
        }
    }
}
///Current Operating Mode. Usually starts in User mode.<br>
///Each operating mode has its own copy of Program Status Register (PSR for short)
#[derive(Copy, Clone, Debug, PartialEq)]
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
    ///Z set
    EQ,
    ///Z clear
    NE,
    ///C set
    CS,
    ///C clear
    CC,
    ///N set(minus)
    MI,
    ///N clear(plus)
    PL,
    ///V set
    VS,
    ///V clear
    VC,
    ///C set and Z clear(unsigned higher)
    HI,
    ///C clear or Z set(unsigned lower or same)
    LS,
    ///N == V(greater or equal)
    GE,
    ///N != V(less than)
    LT,
    ///Z clear and(N == V)(greater than)
    GT,
    ///Z set or(N != V)(less equal)
    LE,
    ///Always
    AL,
    ///Error
    ERR,
}
