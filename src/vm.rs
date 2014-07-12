enum StackItem<u16, String> {
    StackInteger(u16),
    StackString(String)
}

pub fn execute_vm(strict: bool, bytecodes: &[Bytecode], constant_pool: &[Constant<u16>]) {
    let mut stack: Vec<StackItem<int, &str>> = Vec::new();
    let mut instruction_pointer = 0;
    loop {
        match bytecodes[instruction_pointer].opcode {
            Nop => continue,
            ISum => {
                match (stack.pop(), stack.pop()) {
                    (Some(StackInteger(tos)),Some(StackInteger(tos1))) => {
                        stack.push(StackInteger(tos + tos1));
                    },
                    _ => fail!()
                }
            },
            ISub => {
                match (stack.pop(), stack.pop()) {
                    (Some(StackInteger(tos)),Some(StackInteger(tos1))) => {
                        stack.push(StackInteger(tos - tos1));
                    },
                    _ => fail!()
                }
            },
            ILoadConst => {
                match constant_pool[((bytecodes[instruction_pointer].firstbyte.unwrap() << 8) as uint) + (bytecodes[instruction_pointer].secondbyte.unwrap() as uint)] {
                    IntConstant(n) => stack.push(StackInteger(n))//,
                }
            },
            IReturn => {
                match stack.pop() {
                    Some(StackInteger(tos)) => {
                        println!("{}", tos);
                        return;
                    },
                    _ => fail!()
                }
            }
        }
        instruction_pointer += 1;
    }
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
}

pub enum Constant<u16> {
    IntConstant(u16)
}
