// Operation codes
const OP_RETURN: u8 = 1;
const OP_CONSTANT: u8 = 2;
const OP_CONSTANT_LONG: u8 = 3;

// The bytecode itself is a raw byte Vec.
// During execution, we decode bytes into variants of Instruction.
// Byte code Vec could also be a Vec<Instruction>, but that would waste a lot of space in the byte code.
// Not quite sure which way is optimal, since the decoding adds extra overhead as well.
enum Instruction {
    OpReturn,
    OpConstant(u8),
    OpConstantLong(u32),
}

#[derive(Debug)]
struct Line {
    // Last instruction index that originated from this line.
    end: usize,
    line: u32,
}

#[derive(Debug)]
struct Chunk {
    code: Vec<u8>,
    constants: Vec<u32>,
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
            end: self.code.len(),
            line,
        })
    }

    fn update_line(&mut self) {
        if let Some(line) = self.lines.last_mut() {
            line.end = self.code.len()
        }
    }

    fn write_chunk(&mut self, code: u8, line: u32) {
        self.code.push(code);

        match self.lines.last() {
            None => self.new_line(line),
            Some(last) if last.line != line => self.new_line(line),
            Some(last) if last.line == line => self.update_line(),
            _ => (),
        };
    }

    fn write_constant(&mut self, value: u32, line: u32) {
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

// Decodes an instruction starting at index i, returns the instruction and its size in bytes.
fn decode(code: &Vec<u8>, i: usize) -> Option<(usize, Instruction)> {
    let op = code[i];
    match op {
        OP_CONSTANT => Some((2, Instruction::OpConstant(code[i + 1]))),
        OP_CONSTANT_LONG => Some((
            5,
            Instruction::OpConstantLong(u32::from_be_bytes([
                code[i + 1],
                code[i + 2],
                code[i + 3],
                code[i + 4],
            ])),
        )),
        OP_RETURN => Some((1, Instruction::OpReturn)),
        _ => None,
    }
}

fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut line_info = chunk.lines.iter();
    let mut cur_line = line_info.next().unwrap();

    let mut offset = 0;
    let len = chunk.code.len();

    while offset < len {
        print!("{:04} ", offset);
        let is_new_line = offset >= cur_line.end;
        if is_new_line {
            cur_line = line_info.next().unwrap();
        }

        if offset == 0 || is_new_line {
            print!("{:4} ", cur_line.line);
        } else {
            print!("   | ");
        }

        offset += disassemble_instruction(&chunk, offset);
    }
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    let (size, inst) = decode(&chunk.code, offset).expect("Decode problem");
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
            "OpConstant {} '{}'",
            index,
            chunk
                .constants
                .get(index as usize)
                .expect("error getting constant for OpConstant")
        ),
        Instruction::OpReturn => println!("OpReturn"),
    }

    size
}

fn main() {
    println!("Hello!");
    let mut chunk = Chunk::new();

    chunk.write_constant(1, 123);
    chunk.write_constant(2, 123);
    chunk.write_chunk(OP_RETURN, 123);

    chunk.write_constant(3, 124);
    chunk.write_constant(4, 124);
    chunk.write_chunk(OP_RETURN, 124);

    println!("{:?}", chunk);

    disassemble_chunk(&chunk, "test chunk");
}
