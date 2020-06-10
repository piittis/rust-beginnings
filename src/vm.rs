// The bytecode itself is Vec<u8>.
// During execution, we decode bytes into variants of Instruction.
// Byte code Vec could also be a Vec<Instruction>, but that would waste space in the byte code.
// Not quite sure which way is optimal, since the decoding adds extra overhead as well.
use crate::chunk::Chunk;
use crate::debug;
use crate::instruction::decode;
use crate::instruction::Instruction;
use crate::instruction::Value;

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

#[derive(Debug)]
pub struct VM<'a> {
    chunk: &'a Chunk,
    ip: usize,
    stack: Vec<Value>,
}

impl<'a> VM<'a> {
    pub fn interpret(chunk: &'a Chunk) -> InterpretResult {
        let mut vm = VM {
            ip: 0,
            chunk,
            stack: Vec::new(),
        };
        vm.run();
        InterpretResult::Ok
    }

    fn run(&mut self) {
        let len = self.chunk.code.len();
        while self.ip < len {
            // for debug
            print!("          ");
            for val in &self.stack {
                print!("[ {} ]", val);
            }
            println!("");

            let (bytes, inst) = decode(&self.chunk.code, self.ip);

            debug::disassemble_instruction(self.chunk, self.ip);

            match inst {
                Instruction::OpConstant(index) => {
                    self.stack.push(self.chunk.constants[index as usize])
                }
                Instruction::OpConstantLong(index) => {
                    self.stack.push(self.chunk.constants[index as usize])
                }
                Instruction::OpReturn => println!("{}", self.stack.pop().unwrap()),
                Instruction::OpAdd => {
                    let (b, a) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
                    self.stack.push(a + b);
                }
                Instruction::OpSubtract => {
                    let (b, a) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
                    self.stack.push(a - b);
                }
                Instruction::OpMultiply => {
                    let (b, a) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
                    self.stack.push(a * b);
                }
                Instruction::OpDivide => {
                    let (b, a) = (self.stack.pop().unwrap(), self.stack.pop().unwrap());
                    self.stack.push(a / b);
                }
                Instruction::OpNegate => {
                    let top = self.stack.pop().unwrap();
                    self.stack.push(-top);
                }
            }
            self.ip += bytes;
        }
    }

    fn resetStack(&mut self) {
        self.stack.clear();
        // TODO reset capacity?
    }
}
