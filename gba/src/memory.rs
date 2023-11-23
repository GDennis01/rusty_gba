use arm7tdmi::cpu::MemoryInterface;

///Simple GBA Memory representation
pub struct Memory {
    //memory is byte addressable, not word addressable
    //general internal memory
    pub bios: Box<[u8; 16 * 1024]>,      //16KBytes, 0 to 0x000_03FFF
    board_wram: Box<[u8; 256 * 1024]>,   //256KBytes, 0x0200_0000 to 0x0203_FFFF
    pub chip_wram: Box<[u8; 32 * 1024]>, //32KBytes, 0x0300_0000 to 0x0300_7FFF
    io_registers: Box<[u8; 1023]>,       //1KByte, 0x0400_0000 to 0x0400_03FE
    //internal display memory
    palette_ram: Box<[u8; 1024]>,    //1KByte, 0x0500_0000 to 0x0500_03FF
    video_ram: Box<[u8; 96 * 1024]>, //96KBytes, 0x0600_0000 to 0x0601_7FFF
    obj_attributes: Box<[u8; 1024]>, //1Kbyte, 0x0700_0000 to 0x07000_03FF
    //external memory, it's actually only 1 region, but it's split into 3 only for wait state
    gamepakrom1: Box<[u8; 32 * 1024 * 1024]>, //32MB, 0x0800_0000 to 0x09FF_FFFF
    gamepakrom2: Box<[u8; 32 * 1024 * 1024]>, //32MB, 0x0A00_0000 to 0x0BFF_FFFF
    gamepakrom3: Box<[u8; 32 * 1024 * 1024]>, //32MB, 0x0C00_0000 to 0x0DFF_FFFF
    gamepaksram: Box<[u8; 64 * 1024]>,        //64KBytes, 0x0E00_0000 to 0x0E00_FFFF
}
impl Memory {
    pub fn init_bios(&mut self, data: Vec<u8>) {
        let len = self.bios.len();
        print!("{} e  {}", data.len(), len);
        self.bios[0..len].copy_from_slice(&data[0..len]);
    }
    pub fn dbg_dump(&self) {
        for item in self.bios.clone().chunks(4).into_iter() {
            print!("{:#X} ", item[0]);
            print!("{:#X} ", item[1]);
            print!("{:#X} ", item[2]);
            print!("{:#X} ", item[3]);
            println!();
        }
    }
}
impl Default for Memory {
    fn default() -> Self {
        Memory {
            bios: vec![0; 16 * 1024].into_boxed_slice().try_into().unwrap(),
            board_wram: vec![0; 256 * 1024].into_boxed_slice().try_into().unwrap(),
            chip_wram: vec![0; 32 * 1024].into_boxed_slice().try_into().unwrap(),
            io_registers: vec![0; 1023].into_boxed_slice().try_into().unwrap(),
            palette_ram: vec![0; 1024].into_boxed_slice().try_into().unwrap(),
            video_ram: vec![0; 96 * 1024].into_boxed_slice().try_into().unwrap(),
            obj_attributes: vec![0; 1024].into_boxed_slice().try_into().unwrap(),
            gamepakrom1: vec![0; 32 * 1024 * 1024]
                .into_boxed_slice()
                .try_into()
                .unwrap(),
            gamepakrom2: vec![0; 32 * 1024 * 1024]
                .into_boxed_slice()
                .try_into()
                .unwrap(),
            gamepakrom3: vec![0; 32 * 1024 * 1024]
                .into_boxed_slice()
                .try_into()
                .unwrap(),
            gamepaksram: vec![0; 64 * 1024].into_boxed_slice().try_into().unwrap(),
        }
    }
}
//TODO: handle the case where youd read/write out of bounds for each memory region
//TODO: bios and rom cannot be written
impl MemoryInterface for Memory {
    fn new() -> Self {
        Memory::default()
    }
    fn read_8(&self, address: u32) -> u8 {
        match address {
            // 0x0000_0000..=0x000_03FFF => self.bios[address as usize],
            // 0x0200_0000..=0x0203_FFFF => self.board_wram[(address - 0x3_FFFF) as usize],
            // 0x0300_0000..=0x0300_7FFF => self.chip_wram[(address - 0x7FFF) as usize],
            // 0x0400_0000..=0x0400_03FE => self.io_registers[(address - 0x3FE) as usize],
            // 0x0500_0000..=0x0500_03FF => self.palette_ram[(address - 0x3FE) as usize],
            // 0x0600_0000..=0x0601_7FFF => self.video_ram[(address - 0x1_7FFF) as usize],
            // 0x0700_0000..=0x0700_03FF => self.obj_attributes[(address - 0x3FF) as usize],
            // 0x0800_0000..=0x09FF_FFFF => self.gamepakrom1[(address - 0x1FF_FFFF) as usize],
            // 0x0A00_0000..=0x0BFF_FFFF => self.gamepakrom2[(address - 0x1FF_FFFF) as usize],
            // 0x0C00_0000..=0x0DFF_FFFF => self.gamepakrom3[(address - 0x1FF_FFFF) as usize],
            // 0x0E00_0000..=0x0E00_FFFF => self.gamepaksram[(address - 0xFFFF) as usize],
            0x0000_0000..=0x000_03FFF => self.bios[address as usize],
            0x0200_0000..=0x0203_FFFF => self.board_wram[(address - 0x0200_0000) as usize],
            0x0300_0000..=0x0300_7FFF => self.chip_wram[(address - 0x0300_0000) as usize],
            0x0400_0000..=0x0400_03FE => self.io_registers[(address - 0x0400_0000) as usize],
            0x0500_0000..=0x0500_03FF => self.palette_ram[(address - 0x0500_0000) as usize],
            0x0600_0000..=0x0601_7FFF => self.video_ram[(address - 0x0600_0000) as usize],
            0x0700_0000..=0x0700_03FF => self.obj_attributes[(address - 0x0700_0000) as usize],
            0x0800_0000..=0x09FF_FFFF => self.gamepakrom1[(address - 0x0800_0000) as usize],
            0x0A00_0000..=0x0BFF_FFFF => self.gamepakrom2[(address - 0x0A00_0000) as usize],
            0x0C00_0000..=0x0DFF_FFFF => self.gamepakrom3[(address - 0x0C00_0000) as usize],
            0x0E00_0000..=0x0E00_FFFF => self.gamepaksram[(address - 0x0E00_0000) as usize],
            _ => panic!("Invalid address: {:#X}", address),
        }
    }
    fn read_16(&self, address: u32) -> u16 {
        u16::from_le_bytes([self.read_8(address), self.read_8(address + 1)])
    }
    ///Returns the 32 bit value(stored in little endian) at the given address
    fn read_32(&self, address: u32) -> u32 {
        u32::from_le_bytes([
            self.read_8(address),
            self.read_8(address + 1),
            self.read_8(address + 2),
            self.read_8(address + 3),
        ])
    }
    fn write_8(&mut self, address: u32, data: u8) {
        match address {
            // 0x0000_0000..=0x000_03FFF => self.bios[address as usize] = data,
            // 0x0200_0000..=0x0203_FFFF => self.board_wram[(address - 0x3_FFFF) as usize] = data,
            // 0x0300_0000..=0x0300_7FFF => self.chip_wram[(address - 0x7FFF) as usize] = data,
            // 0x0400_0000..=0x0400_03FE => self.io_registers[(address - 0x3FE) as usize] = data,
            // 0x0500_0000..=0x0500_03FF => self.palette_ram[(address - 0x3FE) as usize] = data,
            // 0x0600_0000..=0x0601_7FFF => self.video_ram[(address - 0x1_7FFF) as usize] = data,
            // 0x0700_0000..=0x0700_03FF => self.obj_attributes[(address - 0x3FF) as usize] = data,
            // 0x0800_0000..=0x09FF_FFFF => self.gamepakrom1[(address - 0x1FF_FFFF) as usize] = data,
            // 0x0A00_0000..=0x0BFF_FFFF => self.gamepakrom2[(address - 0x1FF_FFFF) as usize] = data,
            // 0x0C00_0000..=0x0DFF_FFFF => self.gamepakrom3[(address - 0x1FF_FFFF) as usize] = data,
            // 0x0E00_0000..=0x0E00_FFFF => self.gamepaksram[(address - 0xFFFF) as usize] = data,
            0x0000_0000..=0x000_03FFF => self.bios[address as usize] = data,
            0x0200_0000..=0x0203_FFFF => self.board_wram[(address - 0200_0000) as usize] = data,
            0x0300_0000..=0x0300_7FFF => self.chip_wram[(address - 0x0300_0000) as usize] = data,
            0x0400_0000..=0x0400_03FE => self.io_registers[(address - 0x0400_0000) as usize] = data,
            0x0500_0000..=0x0500_03FF => self.palette_ram[(address - 0x0500_0000) as usize] = data,
            0x0600_0000..=0x0601_7FFF => self.video_ram[(address - 0x0600_0000) as usize] = data,
            0x0700_0000..=0x0700_03FF => {
                self.obj_attributes[(address - 0x0700_0000) as usize] = data
            }
            0x0800_0000..=0x09FF_FFFF => self.gamepakrom1[(address - 0x0800_0000) as usize] = data,
            0x0A00_0000..=0x0BFF_FFFF => self.gamepakrom2[(address - 0x0A00_0000) as usize] = data,
            0x0C00_0000..=0x0DFF_FFFF => self.gamepakrom3[(address - 0x0C00_0000) as usize] = data,
            0x0E00_0000..=0x0E00_FFFF => self.gamepaksram[(address - 0x0E00_0000) as usize] = data,
            _ => panic!("Invalid address: {:#X}", address),
        }
    }
    fn write_16(&mut self, address: u32, data: u16) {
        // todo!();
        self.write_8(address, data as u8);
        self.write_8(address + 1, (data >> 8) as u8);
    }
    fn write_32(&mut self, address: u32, data: u32) {
        // todo!();
        self.write_8(address + 0, (data) as u8);
        self.write_8(address + 1, (data >> 8) as u8);
        self.write_8(address + 2, (data >> 16) as u8);
        self.write_8(address + 3, (data >> 24) as u8);
    }
}
