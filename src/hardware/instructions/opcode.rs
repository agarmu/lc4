//! An `instruction` is a command which tells the CPU to do some fundamental task,
//! such as add two numbers.
//! super have both an `opcode` which indicates the kind of task to perform
//! and a set of `parameters` which provide inputs to the task being performed.
//! Each `opcode` represents one task that the CPU "knows" how to do.
//! There are just 16 opcodes in LC-3. Everything the computer can calculate is some
//! sequence of these simple super. Each instruction is 16 bits long,
//! with the left 4 bits storing the opcode.
//! The rest of the bits are used to store the parameters.

/// Each `opcode` represents one task that the CPU "knows" how to do.
/// There are just 16 opcodes in LC-3. Everything the computer can calculate is some
/// sequence of these simple super. Each instruction is 16 bits long,
/// with the left 4 bits storing the opcode.
/// The rest of the bits are used to store the parameters.
use crate::hardware::memory::Memory;
use crate::hardware::Registers;

#[derive(PartialEq, Debug)]
pub enum OpCode {
    /// `Br` is an `OpCode` for branch.
    Br = 0,
    /// `Add` is an `OpCode` for add.
    Add = 1,
    /// `Ld` is an `OpCode` for load.
    Ld = 2,
    /// `St` is an `OpCode` for store.
    St = 3,
    /// `Jsr` is an `OpCode` for jump register.
    Jsr = 4,
    /// `And` is an `OpCode` for and.
    And = 5,
    /// `Ldr` is an `OpCode` for load register.
    Ldr = 6,
    /// `Str` is an `OpCode` for store register.
    Str = 7,
    /// `Rti` is an unused `OpCode`.
    Rti = 8,
    /// `Not` is an `OpCode` for bitwise not.
    Not = 9,
    /// `Ldi` is an `OpCode` for load indirect.
    Ldi = 10,
    /// `Sti` is an `OpCode` for store indirect.
    Sti = 11,
    /// `Jmp` is an `OpCode` for jump.
    Jmp = 12,
    /// `Res` is a reserved (unused) `OpCode`.
    Res = 13,
    /// `Lea` is an `OpCode` for load effective address.
    Lea = 14,
    /// `Trap` is an `OpCode` for execute trap.
    Trap = 15,
}
impl OpCode {
    /// `get` returns Some(OpCode), when a valid value (between 0 to 15) is passed,
    /// otherwise it returns None.
    pub fn get(op_code: u16) -> Option<OpCode> {
        match op_code {
            0 => Some(OpCode::Br),
            1 => Some(OpCode::Add),
            2 => Some(OpCode::Ld),
            3 => Some(OpCode::St),
            4 => Some(OpCode::Jsr),
            5 => Some(OpCode::And),
            6 => Some(OpCode::Ldr),
            7 => Some(OpCode::Str),
            8 => Some(OpCode::Rti),
            9 => Some(OpCode::Not),
            10 => Some(OpCode::Ldi),
            11 => Some(OpCode::Sti),
            12 => Some(OpCode::Jmp),
            13 => Some(OpCode::Res),
            14 => Some(OpCode::Lea),
            15 => Some(OpCode::Trap),
            _ => None,
        }
    }
}

pub fn execute_instruction(instr: u16, mut registers: &mut Registers, mut memory: &mut Memory) {
    //extract op_code from the instruction
    let op_code = extract_op_code(&instr);
    //match op_code and execute instruction
    match op_code {
        Some(OpCode::Add) => super::add::add(instr, &mut registers),
        Some(OpCode::And) => super::and::and(instr, &mut registers),
        Some(OpCode::Not) => super::not::not(instr, &mut registers),
        Some(OpCode::Br) => super::br::br(instr, &mut registers),
        Some(OpCode::Jmp) => super::jmp::jmp(instr, &mut registers),
        Some(OpCode::Jsr) => super::jsr::jsr(instr, &mut registers),
        Some(OpCode::Ld) => super::ld::ld(instr, &mut registers, &mut memory),
        Some(OpCode::Ldi) => super::ldi::ldi(instr, &mut registers, &mut memory),
        Some(OpCode::Ldr) => super::ldr::ldr(instr, &mut registers, &mut memory),
        Some(OpCode::Lea) => super::lea::lea(instr, &mut registers),
        Some(OpCode::St) => super::st::st(instr, &mut registers, &mut memory),
        Some(OpCode::Sti) => super::sti::sti(instr, &mut registers, &mut memory),
        Some(OpCode::Str) => super::str::str(instr, &mut registers, &mut memory),
        Some(OpCode::Trap) => super::trap::trap(instr, &mut registers, &mut memory),
        _ => {}
    }
}

//Each instruction is 16 bits long, with the left 4 bits storing the opcode.
//The rest of the bits are used to store the parameters.
//To extract left 4 bits out of the instruction, we'll use ">>" shift-right
//operator and shift first 4 bits 12 positions towards right
fn extract_op_code(instruction: &u16) -> Option<OpCode> {
    OpCode::get(instruction >> 12)
}

#[cfg(test)]
mod extract_op_code_test {
    use super::*;
    #[test]
    fn extract_test() {
        let four = 16384;
        assert_eq!(Some(OpCode::Jsr), extract_op_code(&four));
    }
}

#[cfg(test)]
mod op_code_test {
    use super::*;
    #[test]
    fn op_codes_initial_values() {
        assert_eq!(Some(OpCode::Br), OpCode::get(0));
        assert_eq!(Some(OpCode::Add), OpCode::get(1));
        assert_eq!(Some(OpCode::Ld), OpCode::get(2));
        assert_eq!(Some(OpCode::St), OpCode::get(3));
        assert_eq!(Some(OpCode::Jsr), OpCode::get(4));
        assert_eq!(Some(OpCode::And), OpCode::get(5));
        assert_eq!(Some(OpCode::Ldr), OpCode::get(6));
        assert_eq!(Some(OpCode::Str), OpCode::get(7));
        assert_eq!(Some(OpCode::Rti), OpCode::get(8));
        assert_eq!(Some(OpCode::Not), OpCode::get(9));
        assert_eq!(Some(OpCode::Ldi), OpCode::get(10));
        assert_eq!(Some(OpCode::Sti), OpCode::get(11));
        assert_eq!(Some(OpCode::Jmp), OpCode::get(12));
        assert_eq!(Some(OpCode::Res), OpCode::get(13));
        assert_eq!(Some(OpCode::Lea), OpCode::get(14));
        assert_eq!(Some(OpCode::Trap), OpCode::get(15));
    }
}
