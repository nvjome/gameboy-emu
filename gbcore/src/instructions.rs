//! Instructions
//! 
//! All the CPU instructions, sorted into blocks based on the 2 MSBs of the opcode.
//! Block 1 and 2 are easy and easily decoded. Blocks 0 and 3, not so much...
//! Luckily, the bitmatch crate can help with the pattern matching!
//! The efficiency impact of this is uncertain, but it sure is convenient.

use crate::CPU;
use anyhow::{anyhow, Result};
use bitmatch::bitmatch;

/// Block CB contains some assorted instructions with 2 distinct patterns.
/// These instructions are only accessible using the prefix byte 0xCB.
pub(super) fn blockcb(cpu: &mut CPU, opcode: u8) -> Result<()> {
    todo!()
}

/// Block 0 contains an assortment of instructions
#[bitmatch]
pub(super) fn block0(cpu: &mut CPU, opcode: u8) -> Result<()> {
    #[bitmatch]
    match opcode {
        "00000000" => (), // NOP

        "00dd0001" => { // LD r16, imm16
            match d {
                        0 => cpu.bc.set_pair(cpu.memory.fetch_two_bytes()?),
                        1 => cpu.de.set_pair(cpu.memory.fetch_two_bytes()?),
                        2 => cpu.hl.set_pair(cpu.memory.fetch_two_bytes()?),
                        3 => cpu.memory.stack_pointer = cpu.memory.fetch_two_bytes()?,
                        _ => return Err(anyhow!("Undefined opcode: {}", opcode))
                    }
        },

        "00dd0010" => { // LD [r16mem], a
            match d {
                _ => return Err(anyhow!("Undefined opcode: {}", opcode))
            }
        },

        "00ss1010" => { // LD a, [r16mem]
            match s {
                _ => return Err(anyhow!("Undefined opcode: {}", opcode))
            }
        },

        "00001000" => todo!(), // LC [imm16], sp

        "00oo0011" => { // INC r16
            todo!()
        },

        "00oo1011" => { // DEC r16
            todo!()
        },

        "00oo1001" => { // ADD hl, r16
            todo!()
        },

        "11ooo100" => { // INC r8
            todo!()
        },

        "00ooo101" => { // DEC r8
            todo!()
        },

        "00ddd110" => { // LD r8, imm8
            todo!()
        },

        "00011000" => { //JR imm8
            todo!()
        },

        "001cc000" => { // JR cond, imm8
            todo!()
        },

        "00010000" => todo!(), // STOP

        _ => return Err(anyhow!("Undefined opcode: {}", opcode))
    }
    Ok(())
}

/// Block 1 contains 8-bit register loads with an easily decoded pattern
#[bitmatch]
pub(super) fn block1(cpu: &mut CPU, opcode: u8) -> Result<()> {
    #[bitmatch]
    let "??dddsss" = opcode;
    // let (destr8, srcr8) = ((opcode >> 3) & 0x7, opcode & 0x7);
    match (d, s) {
        // LD b, r8
        (0, 0) => (),
        (0, 1) => cpu.bc.high = cpu.bc.low,
        (0, 2) => cpu.bc.high = cpu.de.high,
        (0, 3) => cpu.bc.high = cpu.de.low,
        (0, 4) => cpu.bc.high = cpu.hl.high,
        (0, 5) => cpu.bc.high = cpu.hl.low,
        (0, 6) => cpu.bc.high = cpu.memory.read_byte(cpu.hl.get_pair())?,
        (0, 7) => cpu.bc.high = cpu.af.high,

        // LD c, r8
        (1, 0) => cpu.bc.low = cpu.bc.high,
        (1, 1) => (),
        (1, 2) => cpu.bc.low = cpu.de.high,
        (1, 3) => cpu.bc.low = cpu.de.low,
        (1, 4) => cpu.bc.low = cpu.hl.high,
        (1, 5) => cpu.bc.low = cpu.hl.low,
        (1, 6) => cpu.bc.low = cpu.memory.read_byte(cpu.hl.get_pair())?,
        (1, 7) => cpu.bc.low = cpu.af.high,

        // LD d, r8
        (2, 0) => cpu.de.high = cpu.bc.high,
        (2, 1) => cpu.de.high = cpu.bc.low,
        (2, 2) => (),
        (2, 3) => cpu.de.high = cpu.de.low,
        (2, 4) => cpu.de.high = cpu.hl.high,
        (2, 5) => cpu.de.high = cpu.hl.low,
        (2, 6) => cpu.de.high = cpu.memory.read_byte(cpu.hl.get_pair())?,
        (2, 7) => cpu.de.high = cpu.af.high,

        // LD e, r8
        (3, 0) => cpu.de.low = cpu.bc.high,
        (3, 1) => cpu.de.low = cpu.bc.low,
        (3, 2) => cpu.de.low = cpu.de.high,
        (3, 3) => (),
        (3, 4) => cpu.de.low = cpu.hl.high,
        (3, 5) => cpu.de.low = cpu.hl.low,
        (3, 6) => cpu.de.low = cpu.memory.read_byte(cpu.hl.get_pair())?,
        (3, 7) => cpu.de.low = cpu.af.high,

        // LD h, r8
        (4, 0) => cpu.hl.high = cpu.bc.high,
        (4, 1) => cpu.hl.high = cpu.bc.low,
        (4, 2) => cpu.hl.high = cpu.de.high,
        (4, 3) => cpu.hl.high = cpu.de.low,
        (4, 4) => (),
        (4, 5) => cpu.hl.high = cpu.hl.low,
        (4, 6) => cpu.hl.high = cpu.memory.read_byte(cpu.hl.get_pair())?,
        (4, 7) => cpu.hl.high = cpu.af.high,

        // LD l, r8
        (5, 0) => cpu.hl.low = cpu.bc.high,
        (5, 1) => cpu.hl.low = cpu.bc.low,
        (5, 2) => cpu.hl.low = cpu.de.high,
        (5, 3) => cpu.hl.low = cpu.de.low,
        (5, 4) => cpu.hl.low = cpu.hl.high,
        (5, 5) => (),
        (5, 6) => cpu.hl.low = cpu.memory.read_byte(cpu.hl.get_pair())?,
        (5, 7) => cpu.hl.low = cpu.af.high,

        // LD [hl], r8
        (6, 0) => cpu.memory.write_byte(cpu.hl.get_pair(), cpu.bc.high)?,
        (6, 1) => cpu.memory.write_byte(cpu.hl.get_pair(), cpu.bc.low)?,
        (6, 2) => cpu.memory.write_byte(cpu.hl.get_pair(), cpu.de.high)?,
        (6, 3) => cpu.memory.write_byte(cpu.hl.get_pair(), cpu.de.low)?,
        (6, 4) => cpu.memory.write_byte(cpu.hl.get_pair(), cpu.hl.high)?,
        (6, 5) => cpu.memory.write_byte(cpu.hl.get_pair(), cpu.hl.low)?,
        (6, 6) => todo!(), // HALT
        (6, 7) => cpu.memory.write_byte(cpu.hl.get_pair(), cpu.af.high)?,

        // LD a, r8
        (7, 0) => cpu.af.high = cpu.bc.high,
        (7, 1) => cpu.af.high = cpu.bc.low,
        (7, 2) => cpu.af.high = cpu.de.high,
        (7, 3) => cpu.af.high = cpu.de.low,
        (7, 4) => cpu.af.high = cpu.hl.high,
        (7, 5) => cpu.af.high = cpu.hl.low,
        (7, 6) => cpu.af.high = cpu.memory.read_byte(cpu.hl.get_pair())?,
        (7, 7) => (),

        (_, _) => return Err(anyhow!("Undefined opcode: {}", opcode))
    }
    Ok(())
}

/// Block 2 contains 8-bit arithmetic with an easily decoded pattern
#[bitmatch]
pub(super) fn block2(cpu: &mut CPU, opcode: u8) -> Result<()> {
    #[bitmatch]
    match opcode {
        "10000ooo" => { // ADD a, r8
            todo!()
        },

        "10001ooo" => { // ADC a, r8
            todo!()
        },

        "10010ooo" => { // SUB a, r8
            todo!()
        },

        "10011ooo" => { // SBC a, r8
            todo!()
        },

        "10100ooo" => { // AND a, r8
            todo!()
        },

        "10101ooo" => { // XOR a, r8
            todo!()
        },

        "10110ooo" => { // OR a, r8
            todo!()
        },

        "10111ooo" => { // CP a, r8
            todo!()
        },

        _ => return Err(anyhow!("Undefined opcode: {}", opcode))
    }
    Ok(())
}

/// Block 3 again contains an assortment instructions
#[bitmatch]
pub(super) fn block3(cpu: &mut CPU, opcode: u8) -> Result<()> {
    #[bitmatch]
    match opcode {
        _ => return Err(anyhow!("Undefined opcode: {}", opcode))
    }
    Ok(())
}
