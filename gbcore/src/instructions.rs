//! Instructions
//! 
//! All the CPU instructions, sorted into blocks based on the 2 MSBs of the opcode.
//! Block 1 and 2 are easy and easily decoded. Blocks 0 and 3, not so much...
//! Luckily, the bitmatch crate can help with the pattern matching!
//! The efficiency impact of this is uncertain, but it sure is convenient.

use crate::{memory, CPU};
use anyhow::{anyhow, Result};
use bitmatch::bitmatch;

/// Block 0 contains an assortment of instructions.
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

/// Block 1 contains 8-bit register loads with an easily decoded pattern.
#[bitmatch]
pub(super) fn block1(cpu: &mut CPU, opcode: u8) -> Result<i32> {
    #[bitmatch]
    let "??dddsss" = opcode;
    let mut cycles: i32 = 4;

    match (d, s) {
        // LD b, r8
        (0, 0) => (),
        (0, 1) => cpu.bc.high = cpu.bc.low,
        (0, 2) => cpu.bc.high = cpu.de.high,
        (0, 3) => cpu.bc.high = cpu.de.low,
        (0, 4) => cpu.bc.high = cpu.hl.high,
        (0, 5) => cpu.bc.high = cpu.hl.low,
        (0, 6) => {cpu.bc.high = cpu.memory.read_byte(cpu.hl.get_pair())?; cycles = 8;},
        (0, 7) => cpu.bc.high = cpu.af.high,

        // LD c, r8
        (1, 0) => cpu.bc.low = cpu.bc.high,
        (1, 1) => (),
        (1, 2) => cpu.bc.low = cpu.de.high,
        (1, 3) => cpu.bc.low = cpu.de.low,
        (1, 4) => cpu.bc.low = cpu.hl.high,
        (1, 5) => cpu.bc.low = cpu.hl.low,
        (1, 6) => {cpu.bc.low = cpu.memory.read_byte(cpu.hl.get_pair())?; cycles = 8;},
        (1, 7) => cpu.bc.low = cpu.af.high,

        // LD d, r8
        (2, 0) => cpu.de.high = cpu.bc.high,
        (2, 1) => cpu.de.high = cpu.bc.low,
        (2, 2) => (),
        (2, 3) => cpu.de.high = cpu.de.low,
        (2, 4) => cpu.de.high = cpu.hl.high,
        (2, 5) => cpu.de.high = cpu.hl.low,
        (2, 6) => {cpu.de.high = cpu.memory.read_byte(cpu.hl.get_pair())?; cycles = 8;},
        (2, 7) => cpu.de.high = cpu.af.high,

        // LD e, r8
        (3, 0) => cpu.de.low = cpu.bc.high,
        (3, 1) => cpu.de.low = cpu.bc.low,
        (3, 2) => cpu.de.low = cpu.de.high,
        (3, 3) => (),
        (3, 4) => cpu.de.low = cpu.hl.high,
        (3, 5) => cpu.de.low = cpu.hl.low,
        (3, 6) => {cpu.de.low = cpu.memory.read_byte(cpu.hl.get_pair())?; cycles = 8;},
        (3, 7) => cpu.de.low = cpu.af.high,

        // LD h, r8
        (4, 0) => cpu.hl.high = cpu.bc.high,
        (4, 1) => cpu.hl.high = cpu.bc.low,
        (4, 2) => cpu.hl.high = cpu.de.high,
        (4, 3) => cpu.hl.high = cpu.de.low,
        (4, 4) => (),
        (4, 5) => cpu.hl.high = cpu.hl.low,
        (4, 6) => {cpu.hl.high = cpu.memory.read_byte(cpu.hl.get_pair())?; cycles = 8;},
        (4, 7) => cpu.hl.high = cpu.af.high,

        // LD l, r8
        (5, 0) => cpu.hl.low = cpu.bc.high,
        (5, 1) => cpu.hl.low = cpu.bc.low,
        (5, 2) => cpu.hl.low = cpu.de.high,
        (5, 3) => cpu.hl.low = cpu.de.low,
        (5, 4) => cpu.hl.low = cpu.hl.high,
        (5, 5) => (),
        (5, 6) => {cpu.hl.low = cpu.memory.read_byte(cpu.hl.get_pair())?; cycles = 8;},
        (5, 7) => cpu.hl.low = cpu.af.high,

        // LD [hl], r8
        (6, _) => {
            cycles = 8;
            match s {
                0 => cpu.memory.write_byte(cpu.hl.get_pair(), cpu.bc.high)?,
                1 => cpu.memory.write_byte(cpu.hl.get_pair(), cpu.bc.low)?,
                2 => cpu.memory.write_byte(cpu.hl.get_pair(), cpu.de.high)?,
                3 => cpu.memory.write_byte(cpu.hl.get_pair(), cpu.de.low)?,
                4 => cpu.memory.write_byte(cpu.hl.get_pair(), cpu.hl.high)?,
                5 => cpu.memory.write_byte(cpu.hl.get_pair(), cpu.hl.low)?,
                6 => todo!(), // HALT
                7 => cpu.memory.write_byte(cpu.hl.get_pair(), cpu.af.high)?,
                _ => return Err(anyhow!("Undefined opcode: {}", opcode))
            }
    },

        // LD a, r8
        (7, 0) => cpu.af.high = cpu.bc.high,
        (7, 1) => cpu.af.high = cpu.bc.low,
        (7, 2) => cpu.af.high = cpu.de.high,
        (7, 3) => cpu.af.high = cpu.de.low,
        (7, 4) => cpu.af.high = cpu.hl.high,
        (7, 5) => cpu.af.high = cpu.hl.low,
        (7, 6) => {cpu.af.high = cpu.memory.read_byte(cpu.hl.get_pair())?; cycles = 8;},
        (7, 7) => (),

        (_, _) => return Err(anyhow!("Undefined opcode: {}", opcode))
    }
    Ok(cycles)
}

/// Block 2 contains 8-bit arithmetic with an easily decoded pattern.
#[bitmatch]
pub(super) fn block2(cpu: &mut CPU, opcode: u8) -> Result<i32> {
    let mut cycles:i32 = 4;

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
    Ok(cycles)
}

/// Block 3 again contains an assortment of instructions.
#[bitmatch]
pub(super) fn block3(cpu: &mut CPU, opcode: u8) -> Result<i32> {
    #[bitmatch]
    match opcode {
        "11000110" => { // ADD a, imm8
            todo!()
        },

        "11001110" => { // ADD a, imm8
            todo!()
        },

        "11010110" => { // SUB a, imm8
            todo!()
        },

        "11011110" => { // SBC a, imm8
            todo!()
        },

        "11100110" => { // AND a, imm8
            todo!()
        },

        "11101110" => { // XOR a, imm8
            todo!()
        },

        "11110110" => { // OR a, imm8
            todo!()
        },

        "11111110" => { // CP a, imm8
            todo!()
        },

        "110ccc000" => { // RET cond
            todo!()
        },

        "11001001" => { // RET
            todo!()
        },

        "11011001" => { // RETI
            todo!()
        },

        "110cc010" => { // JP cond, imm16
            todo!()
        },

        "11101001" => { // JP hl
            todo!()
        },

        "110cc100" => { // CALL cond, imm16
            todo!()
        },

        "11001101" => { // CALL imm16
            todo!()
        },

        "11ttt111" => { // RST tgt3
            todo!()
        },

        "11rr0001" => { // POP r16stk
            todo!()
        },

        "11rr0101" => { // PUSH r16stk
            todo!()
        },

        "11100010" => { // LDH [c], a
            cpu.memory.write_byte(0xFF00 + cpu.bc.low as u16, cpu.af.high)?;
            Ok(8)
        },

        "11100000" => { // LDH [imm8], a
            todo!()
        },

        "11101010" => { // LD [imm16], a
            let addr = cpu.memory.fetch_two_bytes()?;
            if (0xFF00..=0xFFFF).contains(&addr) {
                cpu.memory.write_byte(addr, cpu.af.high)?;
            }
            Ok(16)
        },

        "11110010" => { // LDH a, [c]
            cpu.af.high = cpu.memory.read_byte(0xFF00 + cpu.bc.low as u16)?;
            Ok(8)
        },

        "11110000" => { // LDH a, [imm8]
            cpu.af.high = cpu.memory.fetch_byte()?;
            Ok(12)
        },

        "11111010" => { // LD a, [imm16]
            let addr = cpu.memory.fetch_two_bytes()?;
            cpu.af.high = cpu.memory.read_byte(addr)?;
            Ok(16)
        },

        "11101000" => { // ADD sp, imm8
            todo!()
        },

        "11111000" => { // LD hl, sp + imm8
            todo!()
        },

        "11111001" => { // LD sp, hl
            cpu.memory.stack_pointer = cpu.hl.get_pair();
            Ok(8)
        },

        "11110011" => { // DI
            todo!()
        },

        "11111011" => { // EI
            todo!()
        },

        _ => return Err(anyhow!("Undefined opcode: {}", opcode))
    }
}

/// Block CB contains an assortment of instructions with 2 distinct decoding patterns.
/// These instructions are only accessible using the prefix byte 0xCB.
#[bitmatch]
pub(super) fn blockcb(cpu: &mut CPU, opcode: u8) -> Result<i32> {
    let mut cycles: i32 = 8;
    #[bitmatch]
    match opcode {
        "00000ooo" => { // RLC r8
            todo!()
        },

        "00001ooo" => { // RRC r8
            todo!()
        },

        "00010ooo" => { // RL r8
            todo!()
        },

        "00011ooo" => { // RR r8
            todo!()
        },

        "00100ooo" => { // SLA r8
            todo!()
        },

        "00101ooo" => { // SRA r8
            todo!()
        },

        "00110ooo" => { // SWAP r8
            todo!()
        },

        "00111ooo" => { // SRL r8
            todo!()
        },

        "01iiiooo" => { // BIT b3, r8
            todo!()
        },

        "10iiiooo" => { // RES b3, r8
            todo!()
        },

        "11iiiooo" => { // SET b3, r8
            todo!()
        },

        _ => return Err(anyhow!("Undefined opcode: {}", opcode))
    }
    Ok(cycles)
}
