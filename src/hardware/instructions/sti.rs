// use super::sign_extend;
// use crate::hardware::register::Registers;
// use crate::hardware::Memory;
//
// pub fn sti(instr: u16, registers: &mut Registers, memory: &mut Memory) {
//     let dr = (instr >> 9) & 0x7;
//     let pc_offset = sign_extend(instr & 0x1ff, 9);
//     // memory.write(
//     //     mem_read(registers.r_pc  + pc_offset ),
//     //     registers.get(dr),
//     // );
// }
