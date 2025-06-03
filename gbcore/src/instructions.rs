use crate::{registers::RegisterPair, CPU};
use anyhow::{anyhow, Result};

pub(super) fn blockcb(cpu: &mut CPU, opcode: u8) -> Result<()> {
    todo!()
}

pub(super) fn block0(cpu: &mut CPU, opcode: u8) -> Result<()> {
    match opcode {
        0x00 => {
            // NOP
        },
        _ => {
            return Err(anyhow!("Undefined opcode: {}", opcode))
        }
    }
    Ok(())
}

pub(super) fn block1(cpu: &mut CPU, opcode: u8) -> Result<()> {
    let (destr8, srcr8) = ((opcode >> 3) & 0x7, opcode & 0x7);
    match (destr8, srcr8) {
        (0, 1) => cpu.bc.high = cpu.bc.low,
        (0, 2) => cpu.bc.high = cpu.de.high,
        (0, 3) => cpu.bc.high = cpu.de.low,
        (0, 4) => cpu.bc.high = cpu.hl.high,
        (0, 5) => cpu.bc.high = cpu.hl.low,
        (0, 6) => {
            cpu.bc.high = cpu.hl.low;
            cpu.hl.high = 0;
        },
        (0, 7) => cpu.bc.high = cpu.af.high,
        (_, _) => return Err(anyhow!("Undefined opcode: {}", opcode))
    }
    Ok(())
}

pub(super) fn block2(cpu: &mut CPU, opcode: u8) -> Result<()> {
    todo!()
}

pub(super) fn block3(cpu: &mut CPU, opcode: u8) -> Result<()> {
    todo!()
}