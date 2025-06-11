//! Instructions
//! 
//! All the CPU instructions, sorted into blocks based on the 2 MSBs of the opcode.
//! Block 1 and 2 are easy and easily decoded. Blocks 0 and 3, not so much...
//! Luckily, the bitmatch crate can help with the pattern matching!
//! The efficiency impact of this is uncertain, but it sure is convenient.

use crate::CPU;
use anyhow::{anyhow, Result};
use bitmatch::bitmatch;

const ZERO_FLAG: u8 = 1 << 7;
const SUB_FLAG: u8 = 1 << 6;
const HALF_CARRY_FLAG: u8 = 1 << 5;
const CARRY_FLAG: u8 = 1 << 4;

/// Block 0 contains an assortment of instructions.
/// Returns the passed time in machine cycles.
#[bitmatch]
pub(super) fn block0(cpu: &mut CPU, opcode: u8) -> Result<i32> {
    #[bitmatch]
    match opcode {
        "00000000" => Ok(1), // NOP

        "00dd0001" => { // LD r16, imm16
            match d {
                0 => cpu.bc.set_pair(cpu.memory.fetch_two_bytes()?),
                1 => cpu.de.set_pair(cpu.memory.fetch_two_bytes()?),
                2 => cpu.hl.set_pair(cpu.memory.fetch_two_bytes()?),
                3 => cpu.memory.stack_pointer = cpu.memory.fetch_two_bytes()?,
                _ => return Err(anyhow!("Undefined opcode: {}", opcode))
            }
            Ok(3)
        },

        "00dd0010" => { // LD [r16mem], a
            match d {
                0 => {
                    let addr = cpu.bc.get_pair();
                    cpu.memory.write_byte(addr, cpu.af.high)?;
                },
                1 => {
                    let addr = cpu.de.get_pair();
                    cpu.memory.write_byte(addr, cpu.af.high)?;
                },
                2 => {
                    let addr = cpu.hl.get_pair();
                    cpu.memory.write_byte(addr, cpu.af.high)?;
                    cpu.hl.incr_pair();
                },
                3 => {
                    let addr = cpu.hl.get_pair();
                    cpu.memory.write_byte(addr, cpu.af.high)?;
                    cpu.hl.decr_pair();
                }
                _ => return Err(anyhow!("Undefined opcode: {}", opcode))
            }
            Ok(2)
        },

        "00ss1010" => { // LD a, [r16mem]
            match s {
                0 => cpu.af.set_pair(cpu.memory.read_two_bytes(cpu.bc.get_pair())?),
                1 => cpu.af.set_pair(cpu.memory.read_two_bytes(cpu.de.get_pair())?),
                2 => {
                    cpu.af.set_pair(cpu.memory.read_two_bytes(cpu.hl.get_pair())?);
                    cpu.hl.incr_pair();
                },
                3 => {
                    cpu.af.set_pair(cpu.memory.read_two_bytes(cpu.hl.get_pair())?);
                    cpu.hl.decr_pair();
                },
                _ => return Err(anyhow!("Undefined opcode: {}", opcode))
            }
            Ok(2)
        },

        "00001000" => { // LD [imm16], sp
            let addr = cpu.memory.fetch_two_bytes()?;
            cpu.memory.write_two_bytes(addr, cpu.memory.stack_pointer)?;
            Ok(5)
        },

        "00oo0011" => { // INC r16
            todo!()
        },

        "00oo1011" => { // DEC r16
            todo!()
        },

        "00oo1001" => { // ADD hl, r16
            let (result, flags) = match o {
                0 => add16(cpu.hl.get_pair(), cpu.bc.get_pair()),
                1 => add16(cpu.hl.get_pair(), cpu.de.get_pair()),
                2 => add16(cpu.hl.get_pair(), cpu.hl.get_pair()),
                3 => add16(cpu.hl.get_pair(), cpu.memory.stack_pointer),
                _ => return Err(anyhow!("Undefined opcode: {}", opcode))
            };
            cpu.hl.set_pair(result);
            cpu.af.low = flags;
            Ok(8)
        },

        "11ooo100" => { // INC r8
            todo!()
        },

        "00ooo101" => { // DEC r8
            todo!()
        },

        "00ddd110" => { // LD r8, imm8
            let mut cycles = 2;
            match d {
                0 => cpu.bc.high = cpu.memory.fetch_byte()?,
                1 => cpu.bc.low = cpu.memory.fetch_byte()?,
                2 => cpu.de.high = cpu.memory.fetch_byte()?,
                3 => cpu.de.low = cpu.memory.fetch_byte()?,
                4 => cpu.hl.high = cpu.memory.fetch_byte()?,
                5 => cpu.hl.low = cpu.memory.fetch_byte()?,
                6 => {
                    let data = cpu.memory.fetch_byte()?;
                    cpu.memory.write_byte(cpu.hl.get_pair(), data)?;
                    cycles = 3
                },
                7 => cpu.hl.high = cpu.memory.fetch_byte()?,
                _ => return Err(anyhow!("Undefined opcode: {}", opcode))
            }
            Ok(cycles)
        },

        "00000111" => { // RLCA
            todo!()
        },

        "00001111" => { // RRCA
            todo!()
        },

        "00010111" => { // RLA
            todo!()
        },

        "00011111" => { // RRA
            todo!()
        },

        "00100111" => { // DAA
            todo!()
        },

        "00101111" => { // CPL
            todo!()
        },

        "00110111" => { // SCF
            cpu.af.low |= CARRY_FLAG;
            cpu.af.low &= !(SUB_FLAG & HALF_CARRY_FLAG);
            Ok(1)
        },

        "00111111" => { // CCF
            cpu.af.low ^= CARRY_FLAG;
            cpu.af.low &= !(SUB_FLAG & HALF_CARRY_FLAG);
            Ok(1)
        },

        "00011000" => { //JR imm8
            todo!()
        },

        "001cc000" => { // JR cond, imm8
            todo!()
        },

        "00010000" => todo!(), // STOP

        _ => Err(anyhow!("Undefined opcode: {}", opcode))
    }
    // Ok(cycles)
}

/// Block 1 contains 8-bit register loads with an easily decoded pattern.
#[bitmatch]
pub(super) fn block1(cpu: &mut CPU, opcode: u8) -> Result<i32> {
    #[bitmatch]
    let "??dddsss" = opcode;
    let mut cycles: i32 = 1;

    match (d, s) {
        // LD b, r8
        (0, 0) => (),
        (0, 1) => cpu.bc.high = cpu.bc.low,
        (0, 2) => cpu.bc.high = cpu.de.high,
        (0, 3) => cpu.bc.high = cpu.de.low,
        (0, 4) => cpu.bc.high = cpu.hl.high,
        (0, 5) => cpu.bc.high = cpu.hl.low,
        (0, 6) => {cpu.bc.high = cpu.memory.read_byte(cpu.hl.get_pair())?; cycles = 2;},
        (0, 7) => cpu.bc.high = cpu.af.high,

        // LD c, r8
        (1, 0) => cpu.bc.low = cpu.bc.high,
        (1, 1) => (),
        (1, 2) => cpu.bc.low = cpu.de.high,
        (1, 3) => cpu.bc.low = cpu.de.low,
        (1, 4) => cpu.bc.low = cpu.hl.high,
        (1, 5) => cpu.bc.low = cpu.hl.low,
        (1, 6) => {cpu.bc.low = cpu.memory.read_byte(cpu.hl.get_pair())?; cycles = 2;},
        (1, 7) => cpu.bc.low = cpu.af.high,

        // LD d, r8
        (2, 0) => cpu.de.high = cpu.bc.high,
        (2, 1) => cpu.de.high = cpu.bc.low,
        (2, 2) => (),
        (2, 3) => cpu.de.high = cpu.de.low,
        (2, 4) => cpu.de.high = cpu.hl.high,
        (2, 5) => cpu.de.high = cpu.hl.low,
        (2, 6) => {cpu.de.high = cpu.memory.read_byte(cpu.hl.get_pair())?; cycles = 2;},
        (2, 7) => cpu.de.high = cpu.af.high,

        // LD e, r8
        (3, 0) => cpu.de.low = cpu.bc.high,
        (3, 1) => cpu.de.low = cpu.bc.low,
        (3, 2) => cpu.de.low = cpu.de.high,
        (3, 3) => (),
        (3, 4) => cpu.de.low = cpu.hl.high,
        (3, 5) => cpu.de.low = cpu.hl.low,
        (3, 6) => {cpu.de.low = cpu.memory.read_byte(cpu.hl.get_pair())?; cycles = 2;},
        (3, 7) => cpu.de.low = cpu.af.high,

        // LD h, r8
        (4, 0) => cpu.hl.high = cpu.bc.high,
        (4, 1) => cpu.hl.high = cpu.bc.low,
        (4, 2) => cpu.hl.high = cpu.de.high,
        (4, 3) => cpu.hl.high = cpu.de.low,
        (4, 4) => (),
        (4, 5) => cpu.hl.high = cpu.hl.low,
        (4, 6) => {cpu.hl.high = cpu.memory.read_byte(cpu.hl.get_pair())?; cycles = 2;},
        (4, 7) => cpu.hl.high = cpu.af.high,

        // LD l, r8
        (5, 0) => cpu.hl.low = cpu.bc.high,
        (5, 1) => cpu.hl.low = cpu.bc.low,
        (5, 2) => cpu.hl.low = cpu.de.high,
        (5, 3) => cpu.hl.low = cpu.de.low,
        (5, 4) => cpu.hl.low = cpu.hl.high,
        (5, 5) => (),
        (5, 6) => {cpu.hl.low = cpu.memory.read_byte(cpu.hl.get_pair())?; cycles = 2;},
        (5, 7) => cpu.hl.low = cpu.af.high,

        // LD [hl], r8
        (6, _) => {
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
            cycles = 2;
        },

        // LD a, r8
        (7, 0) => cpu.af.high = cpu.bc.high,
        (7, 1) => cpu.af.high = cpu.bc.low,
        (7, 2) => cpu.af.high = cpu.de.high,
        (7, 3) => cpu.af.high = cpu.de.low,
        (7, 4) => cpu.af.high = cpu.hl.high,
        (7, 5) => cpu.af.high = cpu.hl.low,
        (7, 6) => {cpu.af.high = cpu.memory.read_byte(cpu.hl.get_pair())?; cycles = 2;},
        (7, 7) => (),

        (_, _) => return Err(anyhow!("Undefined opcode: {}", opcode))
    }
    Ok(cycles)
}

/// Block 2 contains 8-bit arithmetic with an easily decoded pattern.
#[bitmatch]
pub(super) fn block2(cpu: &mut CPU, opcode: u8) -> Result<i32> {
    let mut cycles = 1;
    #[bitmatch]
    match opcode {
        "10000ooo" => { // ADD a, r8
            (cpu.af.high, cpu.af.low) = match o {
                0 => add8(cpu.af.high, cpu.bc.high),
                1 => add8(cpu.af.high, cpu.bc.low),
                2 => add8(cpu.af.high, cpu.de.high),
                3 => add8(cpu.af.high, cpu.de.low),
                4 => add8(cpu.af.high, cpu.hl.high),
                5 => add8(cpu.af.high, cpu.hl.low),
                6 => {cycles = 2; add8(cpu.af.high, cpu.memory.read_byte(cpu.hl.get_pair())?)},
                7 => add8(cpu.af.high, cpu.af.high),
                _ => return Err(anyhow!("Undefined opcode: {}", opcode))
            }
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
            (cpu.af.high, cpu.af.low) = add8(cpu.af.high, cpu.memory.fetch_byte()?);
            Ok(2)
        },

        "11001110" => { // ADC a, imm8
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
            Ok(2)
        },

        "11100000" => { // LDH [imm8], a
            let addr = cpu.memory.fetch_byte()?;
            cpu.af.high = cpu.memory.read_byte(0xFF00 + addr as u16)?;
            Ok(2)
        },

        "11101010" => { // LD [imm16], a
            let addr = cpu.memory.fetch_two_bytes()?;
            if (0xFF00..=0xFFFF).contains(&addr) {
                cpu.memory.write_byte(addr, cpu.af.high)?;
            }
            Ok(4)
        },

        "11110010" => { // LDH a, [c]
            cpu.af.high = cpu.memory.read_byte(0xFF00 + cpu.bc.low as u16)?;
            Ok(2)
        },

        "11110000" => { // LDH a, [imm8]
            let addr = cpu.memory.fetch_byte()?;
            cpu.af.high = cpu.memory.read_byte(0xFF00 + addr as u16)?;
            Ok(3)
        },

        "11111010" => { // LD a, [imm16]
            let addr = cpu.memory.fetch_two_bytes()?;
            cpu.af.high = cpu.memory.read_byte(addr)?;
            Ok(4)
        },

        "11101000" => { // ADD sp, imm8
            todo!()
        },

        "11111000" => { // LD hl, sp + imm8
            let val = cpu.memory.fetch_byte()? as i8 as i16;
            let sp = cpu.memory.stack_pointer;
            let result = sp.wrapping_add_signed(val);
            let carry = (sp & 0xFF) + (val as u16 & 0xFF) > 0xFF;
            let half_carry = (sp & 0xF) + (val as u16 & 0xF) > 0xF;
            let mut flags = 0;
            if carry {flags |= 0x10}
            if half_carry {flags |= 0x20}
            cpu.hl.set_pair(result);
            cpu.af.low = flags;
            Ok(3)
        },

        "11111001" => { // LD sp, hl
            cpu.memory.stack_pointer = cpu.hl.get_pair();
            Ok(2)
        },

        "11110011" => { // DI
            todo!()
        },

        "11111011" => { // EI
            todo!()
        },

        _ => Err(anyhow!("Undefined opcode: {}", opcode))
    }
}

/// Block CB contains an assortment of instructions with 2 distinct decoding patterns.
/// These instructions are only accessible using the prefix byte 0xCB.
#[bitmatch]
pub(super) fn blockcb(cpu: &mut CPU, opcode: u8) -> Result<i32> {
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

        _ => Err(anyhow!("Undefined opcode: {}", opcode))
    }
}

// Add two unsigned 8-bit values, returning a tuple with the result and flags.
#[bitmatch]
fn add8(lhs: u8, rhs: u8) -> (u8, u8) {
    let result = lhs.wrapping_add(rhs);
    let c = (((lhs as u16 & 0xFF) + (rhs as u16 & 0xFF)) >> 8) as u8;
    let h = ((lhs & 0xF) + (rhs & 0xF)) >> 4;
    let z: u8 = match result {0 => 0, _ => 1};
    let flags = bitpack!("z0hc0000");
    (result, flags)
}

// Add two unsigned 16-bit values, returning a tuple with the result and flags.
#[bitmatch]
fn add16(lhs: u16, rhs: u16) -> (u16, u8) {
    let result = lhs.wrapping_add(rhs);
    let c = (((lhs as u32 & 0xFFFF) + (rhs as u32 & 0xFFFF)) >> 16) as u8;
    let h = ((lhs & 0xFFF) + (rhs & 0xFFF)) >> 12;
    // let z: u8 = match result {0 => 0, _ => 1};
    let flags = bitpack!("00hc0000");
    (result, flags)
}