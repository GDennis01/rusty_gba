use arm7tdmi::cpu::MemoryInterface;
use std::ops::RangeInclusive;
//Debug Implementation of memory

pub struct Memory {
    //memory is byte addressable, not word addressable
    //general internal memory
    bios: [u8; 16 * 1024],        //16KBytes, 0 to 0x000_03FFF
    board_wram: [u8; 256 * 1024], //256KBytes, 0x0200_0000 to 0x0203_FFFF
    chip_wram: [u8; 32 * 1024],   //32KBytes, 0x0300_0000 to 0x0300_7FFF
    io_registers: [u8; 1023],     //1KByte, 0x0400_0000 to 0x0400_03FE
    //internal display memory
    palette_ram: [u8; 1024],    //1KByte, 0x0500_0000 to 0x0500_03FF
    video_ram: [u8; 96 * 1024], //96KBytes, 0x0600_0000 to 0x0601_7FFF
    obj_attributes: [u8; 1024], //1Kbyte, 0x0700_0000 to 0x07000_03FF
    //external memory, it's actually only 1 region, but it's split into 3 only for wait state
    gamepakrom1: [u8; 32 * 1024 * 1024], //32MB, 0x0800_0000 to 0x09FF_FFFF
    gamepakrom2: [u8; 32 * 1024 * 1024], //32MB, 0x0A00_0000 to 0x0BFF_FFFF
    gamepakrom3: [u8; 32 * 1024 * 1024], //32MB, 0x0C00_0000 to 0x0DFF_FFFF
    gamepaksram: [u8; 64 * 1024],        //64KBytes, 0x0E00_0000 to 0x0E00_FFFF
}
impl Default for Memory {
    fn default() -> Self {
        Memory {
            bios: [0; 16 * 1024],
            board_wram: [0; 256 * 1024],
            chip_wram: [0; 32 * 1024],
            io_registers: [0; 1023],
            palette_ram: [0; 1024],
            video_ram: [0; 96 * 1024],
            obj_attributes: [0; 1024],
            gamepakrom1: [0; 32 * 1024 * 1024],
            gamepakrom2: [0; 32 * 1024 * 1024],
            gamepakrom3: [0; 32 * 1024 * 1024],
            gamepaksram: [0; 64 * 1024],
        }
    }
}
impl MemoryInterface for Memory {
    fn new() -> Self {
        Default::default()
    }
    fn read_8(&self, address: u32) -> u8 {
        //TODO: mask address
        match address {
            0x0000_0000..=0x000_03FFF => self.bios[address as usize],
            0x0200_0000..=0x0203_FFFF => self.board_wram[address as usize],
            0x0300_0000..=0x0300_7FFF => self.chip_wram[address as usize],
            0x0400_0000..=0x0400_03FE => self.io_registers[address as usize],
            0x0500_0000..=0x0500_03FF => self.palette_ram[address as usize],
            0x0600_0000..=0x0601_7FFF => self.video_ram[address as usize],
            0x0700_0000..=0x0700_03FF => self.obj_attributes[address as usize],
            0x0800_0000..=0x09FF_FFFF => self.gamepakrom1[address as usize],
            0x0A00_0000..=0x0BFF_FFFF => self.gamepakrom2[address as usize],
            0x0C00_0000..=0x0DFF_FFFF => self.gamepakrom3[address as usize],
            0x0E00_0000..=0x0E00_FFFF => self.gamepaksram[address as usize],
            _ => panic!("Invalid address: {:#X}", address),
        }
    }
    fn read_16(&self, address: u32) -> u16 {
        1
    }
    fn read_32(&self, address: u32) -> u32 {
        1
    }
    fn write_8(&mut self, address: u32, data: u8) {}
    fn write_16(&mut self, address: u32, data: u16) {}
    fn write_32(&mut self, address: u32, data: u32) {}
}
