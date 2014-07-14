enum StackItem<u16, String> {
    StackInteger(u16),
    StackString(String)
}

pub fn execute_vm(strict: bool, bytecodes: &[Bytecode], constant_pool: &[Constant<u16>]) {
    let mut stack: Vec<StackItem<u16, String>> = Vec::new();
    let mut frame_pointers: Vec<uint> = Vec::new();
    let mut instruction_pointer = 0;
    while instruction_pointer <  bytecodes.len() {
        // println!("{}", stack);
        match bytecodes[instruction_pointer].opcode {
            Nop => instruction_pointer += 1,
            ISum => {
                match (stack.pop(), stack.pop()) {
                    (Some(StackInteger(tos)),Some(StackInteger(tos1))) => {
                        stack.push(StackInteger(tos + tos1));
                        instruction_pointer += 1;
                    },
                    _ => fail!()
                }
            },
            ISub => {
                match (stack.pop(), stack.pop()) {
                    (Some(StackInteger(tos)),Some(StackInteger(tos1))) => {
                        stack.push(StackInteger(tos - tos1));
                        instruction_pointer += 1;
                    },
                    _ => fail!()
                }
            },
            ILoadConst => {
                match constant_pool[((bytecodes[instruction_pointer].firstbyte.unwrap() << 8) as uint) + (bytecodes[instruction_pointer].secondbyte.unwrap() as uint)] {
                    IntConstant(n) => {
                        stack.push(StackInteger(n));
                        instruction_pointer += 1;
                    }
                }
            },
            // IReturn => {
            //     match stack.pop() {
            //         Some(StackInteger(tos)) => {
            //             println!("{}", tos);
            //             // instruction_pointer += 1;
            //             return;
            //         },
            //         _ => fail!()
            //     }
            // },
            IReturn => {
                match stack.pop() {
                    Some(StackInteger(tos)) => {
                        match frame_pointers.pop() {
                            Some(stack_address) => {
                                //Pop everything in the current frame until the ret_addr off the stack
                                while stack_address < stack.len() - 1 {
                                    stack.pop();
                                    // println!("{}, {}", stack_address, stack.len());
                                }
                                //grab the ret_addr
                                let probable_return_address = stack.pop();
                                match probable_return_address {
                                    Some(StackInteger(return_address)) => {
                                        instruction_pointer = return_address as uint;
                                    },
                                    _ => fail!()
                                };

                                //push tos
                                stack.push(StackInteger(tos));
                            },
                            None => fail!()
                        }
                    },
                    _ => fail!()
                }
            },
            Goto => {
                instruction_pointer = ((bytecodes[instruction_pointer].firstbyte.unwrap() << 8) as uint) + (bytecodes[instruction_pointer].secondbyte.unwrap() as uint);
            },
            Call => {
                stack.push(StackInteger(instruction_pointer as u16 + 1));
                frame_pointers.push(stack.len() - 1);
                instruction_pointer = ((bytecodes[instruction_pointer].firstbyte.unwrap() << 8) as uint) + (bytecodes[instruction_pointer].secondbyte.unwrap() as uint);
            },
            Print => {
                match stack.pop() {
                    Some(StackInteger(tos)) => {
                        println!("{}", tos);
                        stack.push(StackInteger(tos));
                        instruction_pointer += 1
                    },
                    _ => fail!()
                }
            }
        }
        // println!("{}, {}", instruction_pointer, bytecodes.len());
    }
    // println!("Done");
}

pub struct Bytecode {
    pub opcode: Opcode,
    pub firstbyte: Option<u8>,
    pub secondbyte: Option<u8>
}

pub enum Opcode {
  Nop = 0x00,
  ISum = 0x01,
  ISub = 0x02,
  IReturn = 0x03,
  ILoadConst = 0x04,
  Goto = 0x05,
  Call = 0x06,
  Print = 0x07
}

pub enum Constant<u16> {
    IntConstant(u16)
}
