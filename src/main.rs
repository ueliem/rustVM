use std::io::BufferedReader;
use std::io;
mod vm;

fn main() {
    let const_list: &[vm::Constant<u16>] = [vm::IntConstant(1_u16)];

    let instr_list: &[vm::Bytecode] = [
    vm::Bytecode{opcode: vm::ILoadConst, firstbyte: Some(0_u8), secondbyte: Some(0_u8)},
    vm::Bytecode{opcode: vm::ILoadConst, firstbyte: Some(0_u8), secondbyte: Some(0_u8)},
    vm::Bytecode{opcode: vm::ISum, firstbyte: None, secondbyte: None},
    vm::Bytecode{opcode: vm::IReturn, firstbyte: None, secondbyte: None}
    ];

    vm::execute_vm(true, instr_list, const_list);
}
