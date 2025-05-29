use anyhow::{anyhow, Result};

const MEM_SIZE: usize = 0x10000;

pub struct Memory {
    ram: [u8; MEM_SIZE],
    pub program_counter: u16,
    pub stack_pointer: u16,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl Memory {
    pub fn new(program_counter: u16, stack_pointer: u16) -> Self {
        Self {
            ram: [0; MEM_SIZE],
            program_counter,
            stack_pointer,
        }
    }

    pub fn load_memory(&mut self, buffer: &[u8], start_addr: u16) -> Result<()> {
        if buffer.len() > MEM_SIZE - (start_addr as usize) {
            return Err(anyhow!("Program size exceeds ROM capacity"))
        }
        let start = start_addr as usize;
        let end = (start_addr as usize) + buffer.len();
        self.ram[start..end].copy_from_slice(buffer);
        self.program_counter = start_addr;
        Ok(())
    }

    pub fn fetch_byte(&mut self) -> Result<u8> {
        match self.ram.get(self.program_counter as usize) {
            Some(byte) => {
                match self.program_counter.checked_add(1) {
                    Some(x) => self.program_counter = x,
                    None => return Err(anyhow!("Program counter overflow"))
                }
                Ok(*byte)
            },
            None => Err(anyhow!("Program counter is out of bounds: {}", self.program_counter))
        }
    }

    pub fn fetch_two_bytes(&mut self) -> Result<u16> {
        let data_low = self.fetch_byte()?;
        let data_high = self.fetch_byte()?;
        Ok((data_high as u16) << 8 | data_low as u16)
    }

    pub fn read_byte(&self, address: u16) -> Result<u8> {
        match self.ram.get(address as usize) {
            Some(byte) => Ok(*byte),
            None => Err(anyhow!("Attempted to read outside of RAM at address: {}", address ))
        }
    }

    pub fn read_two_bytes(&self, address: u16) -> Result<u16> {
        let data_low = self.read_byte(address)?;
        let data_high = self.read_byte(address + 1)?;
        Ok((data_high as u16) << 8 | data_low as u16)
    }

    pub fn write_byte(&mut self, address: u16, data: u8) -> Result<()> {
        match self.ram.get_mut(address as usize) {
            Some(byte) => {
                *byte = data;
                Ok(())
            },
            None => Err(anyhow!("Attempted to write outside of RAM at address: {}", address ))
        }
    }

    pub fn write_two_bytes(&mut self, address: u16, data: u16) -> Result<()> {
        self.write_byte(address, (data & 0x00FF) as u8)?;
        self.write_byte(address + 1, ((data & 0xFF00) >> 8) as u8)?;
        Ok(())
    }

    pub fn pop_stack(&mut self) -> Result<u16> {
        let data = self.read_two_bytes(self.stack_pointer)?;
        match self.stack_pointer.checked_add(2) {
            Some(x) => self.stack_pointer = x,
            None => return Err(anyhow!("Stack pointer overflow"))
        }
        Ok(data)
    }

    pub fn push_stack(&mut self, data: u16) -> Result<()>{
        match self.stack_pointer.checked_sub(2) {
            Some(x) => self.stack_pointer = x,
            None => return Err(anyhow!("Stack pointer overflow"))
        }
        self.write_two_bytes(self.stack_pointer, data)?;
        Ok(())
    }
}