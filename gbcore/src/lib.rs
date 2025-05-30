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

// This contains all components of the CPU
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
    pub fn tick(&mut self) -> Result<()> {
        let opcode = self.memory.fetch_byte()?;
        self.execute(opcode)
    }

    fn execute(&mut self, opcode: u8) -> Result<()> {
        // CB prefix
        if opcode == 0xCB {
            instructions::blockcb(self, opcode)?;
            return Ok(())
        }

        let block_code = (opcode >> 6) & 0x03;

        match block_code {
            0 => {
                // Block 0 (00)
                instructions::block0(self, opcode)?;
                Ok(())
            },
            1 => {
                // Block 1 (01)
                instructions::block1(self, opcode)?;
                Ok(())
            },
            2 => {
                // Block 2 (10)
                instructions::block2(self, opcode)?;
                Ok(())
            },
            3 => {
                // Block 3 (11)
                instructions::block3(self, opcode)?;
                Ok(())
            },
            _ => {
                Err(anyhow!("Undefined opcode: {}", opcode))
            }
        }
    }
}
