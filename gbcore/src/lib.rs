//! Game Boy Core.
//! 
//! The emulation core for the CPU, memory, and IO components of the Game Boy, all through a sensible API.
//! Expects a front end application to capture user input, supply ROM data, and render graphics.

mod registers;
mod memory;
mod instructions;

use anyhow::{anyhow, Ok, Result};
use registers::RegisterPair;
use memory::Memory;

const ROM_ADDR: u16 = 0x0100;

/// This contains all components of the CPU
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
            memory: Memory::new(0, 0),
        }
    }

    /// Loads instructions into memory from some slice (probably a Vector)
    pub fn load_rom(&mut self, buffer: &[u8]) -> Result<()> {
        self.memory.load_memory(buffer, ROM_ADDR)
    }

    /// Performs one fetch-execute cycle
    pub fn tick(&mut self) -> Result<i32> {
        let opcode = self.memory.fetch_byte()?;
        let cycles = self.execute(opcode)? / 4;
        Ok(cycles)
    }

    fn execute(&mut self, opcode: u8) -> Result<i32> {
        // CB prefix
        if opcode == 0xCB {
            let opcode2 = self.memory.fetch_byte()?;
            let tcycles = instructions::blockcb(self, opcode2)?;
            return Ok(tcycles)
        }

        let block_code = (opcode >> 6) & 0x03;

        match block_code {
            0 => {
                // Block 0 (00)
                let tcycles = instructions::block0(self, opcode)?;
                Ok(tcycles)
            },
            1 => {
                // Block 1 (01)
                let tcycles = instructions::block1(self, opcode)?;
                Ok(tcycles)
            },
            2 => {
                // Block 2 (10)
                let tcycles = instructions::block2(self, opcode)?;
                Ok(tcycles)
            },
            3 => {
                // Block 3 (11)
                let tcycles = instructions::block3(self, opcode)?;
                Ok(tcycles)
            },
            _ => {
                Err(anyhow!("Undefined opcode: {}", opcode))
            }
        }
    }
}
