#![crate_id = "rustvm"]
use std::io::BufferedReader;
use std::io;
pub mod vm;

/*fn main() {
    let const_list: &[vm::Constant<u16>] = [vm::IntConstant(1_u16), vm::IntConstant(2_u16), vm::IntConstant(3_u16)];

    let instr_list: &[vm::Bytecode] = [
    vm::Bytecode{opcode: vm::Goto, firstbyte: Some(0_u8), secondbyte: Some(6_u8)},//Go to first real instruction of program
    //Subroutine
    vm::Bytecode{opcode: vm::ILoadConst, firstbyte: Some(0_u8), secondbyte: Some(1_u8)},//Load 2
    vm::Bytecode{opcode: vm::ILoadConst, firstbyte: Some(0_u8), secondbyte: Some(1_u8)},//Load 2
    vm::Bytecode{opcode: vm::ISum, firstbyte: None, secondbyte: None},//Sum 2 + 2
    vm::Bytecode{opcode: vm::Print, firstbyte: None, secondbyte: None},//Print 4
    vm::Bytecode{opcode: vm::IReturn, firstbyte: None, secondbyte: None},
    //End Subroutine
    //Begin Program
    vm::Bytecode{opcode: vm::ILoadConst, firstbyte: Some(0_u8), secondbyte: Some(1_u8)},//Load 2
    vm::Bytecode{opcode: vm::ILoadConst, firstbyte: Some(0_u8), secondbyte: Some(2_u8)},//Load 3
    vm::Bytecode{opcode: vm::ISum, firstbyte: None, secondbyte: None},//Sum 2 + 3
    vm::Bytecode{opcode: vm::Print, firstbyte: None, secondbyte: None},//Print 5
    vm::Bytecode{opcode: vm::Call, firstbyte: Some(0_u8), secondbyte: Some(1_u8)},//,//Call subroutine starting at index 1
    vm::Bytecode{opcode: vm::Print, firstbyte: None, secondbyte: None}//Prints 4 (Returned value from subroutine)
    //End Program
    ];

    vm::execute_vm(true, instr_list, const_list, []);
}*/

