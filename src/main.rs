// Operation codes
const OP_RETURN: u8 = 1;
const OP_CONSTANT: u8 = 2;
const OP_CONSTANT_LONG: u8 = 3;
const OP_ADD: u8 = 4;
const OP_SUBTRACT: u8 = 5;
const OP_MULTIPLY: u8 = 6;
const OP_DIVIDE: u8 = 7;
const OP_NEGATE: u8 = 8;

type Value = f64;

// The bytecode itself is Vec<u8>.
// During execution, we decode bytes into variants of Instruction.
// Byte code Vec could also be a Vec<Instruction>, but that would waste space in the byte code.
// Not quite sure which way is optimal, since the decoding adds extra overhead as well.
enum Instruction {
    OpReturn,
    OpConstant(u8),
    OpConstantLong(u32),
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNegate,
}

enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

// Decodes an instruction starting at index ip, returns the instruction and its size in bytes.
fn decode(code: &Vec<u8>, ip: usize) -> (usize, Instruction) {
    let op = code[ip];
    match op {
        OP_CONSTANT => (2, Instruction::OpConstant(code[ip + 1])),
        OP_CONSTANT_LONG => (
            5,
            Instruction::OpConstantLong(u32::from_be_bytes([
                code[ip + 1],
                code[ip + 2],
                code[ip + 3],
                code[ip + 4],
            ])),
        ),
        OP_RETURN => (1, Instruction::OpReturn),
        OP_ADD => (1, Instruction::OpAdd),
        OP_SUBTRACT => (1, Instruction::OpSubtract),
        OP_MULTIPLY => (1, Instruction::OpMultiply),
        OP_DIVIDE => (1, Instruction::OpDivide),
        OP_NEGATE => (1, Instruction::OpNegate),
        _ => panic!("unknown opcode"),
    }
}

#[derive(Debug)]
struct VM<'a> {
    chunk: &'a Chunk,
    ip: usize,
    stack: Vec<Value>,
}

impl<'a> VM<'a> {
    fn interpret(chunk: &'a Chunk) {
        let mut vm = VM {
            ip: 0,
            chunk,
            stack: Vec::new(),
        };
        vm.run();
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

            disassemble_instruction(self.chunk, self.ip);

            let (bytes, inst) = decode(&self.chunk.code, self.ip);

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

#[derive(Debug)]
struct Chunk {
    code: Vec<u8>,
    constants: Vec<Value>,
    lines: Vec<Line>,
}

impl Chunk {
    fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    fn new_line(&mut self, line: u32) {
        self.lines.push(Line {
            start: self.code.len() - 1,
            line,
        })
    }

    fn write_chunk(&mut self, code: u8, line: u32) {
        self.code.push(code);

        match self.lines.last() {
            None => self.new_line(line),
            Some(last) if last.line != line => self.new_line(line),
            _ => (),
        };
    }

    fn write_constant(&mut self, value: Value, line: u32) {
        let len = self.constants.len();
        if len < 256 {
            self.write_chunk(OP_CONSTANT, line);
            self.write_chunk(len as u8, line);
        } else {
            let bytes = value.to_be_bytes();
            self.write_chunk(OP_CONSTANT_LONG, line);
            self.write_chunk(bytes[0], line);
            self.write_chunk(bytes[1], line);
            self.write_chunk(bytes[2], line);
            self.write_chunk(bytes[3], line);
        }

        self.constants.push(value)
    }
}

#[derive(Debug)]
struct Line {
    start: usize,
    line: u32,
}

fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut ip = 0;
    let len = chunk.code.len();

    while ip < len {
        ip += disassemble_instruction(&chunk, ip);
    }
}

fn disassemble_instruction(chunk: &Chunk, ip: usize) -> usize {
    print!("{:04} ", ip);

    if let Ok(l_index) = chunk.lines.binary_search_by(|l| l.start.cmp(&ip)) {
        // New line was started with this instruction, print it
        print!("{:4} ", unsafe { chunk.lines.get_unchecked(l_index).line });
    } else {
        print!("     ");
    }

    let (size, inst) = decode(&chunk.code, ip);
    match inst {
        Instruction::OpConstant(index) => println!(
            "OpConstant {} '{}'",
            index,
            chunk
                .constants
                .get(index as usize)
                .expect("error getting constant for OpConstant")
        ),
        Instruction::OpConstantLong(index) => println!(
            "OpConstantLong {} '{}'",
            index,
            chunk
                .constants
                .get(index as usize)
                .expect("error getting constant for OpConstantLong")
        ),
        Instruction::OpReturn => println!("OpReturn"),
        Instruction::OpAdd => println!("OpAdd"),
        Instruction::OpSubtract => println!("OpSubtract"),
        Instruction::OpMultiply => println!("OpMultiply"),
        Instruction::OpDivide => println!("OpDivide"),
        Instruction::OpNegate => println!("OpNegate"),
    }

    size
}

fn main() {
    let mut chunk = Chunk::new();

    chunk.write_constant(1.2, 123);
    chunk.write_constant(3.4, 123);
    chunk.write_chunk(OP_ADD, 123);

    chunk.write_constant(5.6, 123);
    chunk.write_chunk(OP_DIVIDE, 123);
    chunk.write_chunk(OP_NEGATE, 123);
    chunk.write_chunk(OP_RETURN, 123);

    println!("{:?}", chunk);

    // disassemble_chunk(&chunk, "test chunk");

    VM::interpret(&chunk);
}
