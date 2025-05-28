mod registers;
mod memory;

use anyhow::Result;
use registers::RegisterPair;
use memory::Memory;

const ROM_ADDR: u16 = 0x0100;

pub struct CPU {
    af: RegisterPair,
    bc: RegisterPair,
    de: RegisterPair,
    hl: RegisterPair,
    memory: Memory,
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

impl CPU {
    pub fn new() -> Self {
        Self {
            af: RegisterPair::new(),
            bc: RegisterPair::new(),
            de: RegisterPair::new(),
            hl: RegisterPair::new(),
            memory: Memory::new(),
        }
    }

    pub fn load_rom(&mut self, buffer: &[u8]) -> Result<()> {
        self.memory.load_rom(buffer, ROM_ADDR)
    }

    pub fn tick(&mut self) -> Result<()> {
        let opcode = self.memory.fetch_byte()?;
        //println!("{:#04x}", opcode);
        self.execute(opcode)
    }

    pub fn execute(&mut self, opcode: u8) -> Result<()> {
        // CB prefix
        if opcode == 0xCB {
            todo!()
        }

        let block_code = (opcode & 0xC0) >> 6;

        // Block 0 (00)
        if block_code == 0 {
            todo!()
        }

        // Block 1 (01)
        if block_code == 1 {
            todo!()
        }

        // Block 2 (10)
        if block_code == 2 {
            todo!()
        }

        // Block 3 (11)
        if block_code == 3 {
            todo!()
        }

        todo!()
    }
}

fn block0(cpu: &mut CPU, opcode: u8) {

}

fn block1(cpu: &mut CPU, opcode: u8) {

}

fn block2(cpu: &mut CPU, opcode: u8) {

}

fn block3(cpu: &mut CPU, opcode: u8) {

}