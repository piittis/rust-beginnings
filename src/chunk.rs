use crate::instruction::Value;
use crate::instruction::OP_CONSTANT;

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: Vec<Line>,
}

#[derive(Debug)]
pub struct Line {
    pub start: usize,
    pub line: u32,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn new_line(&mut self, line: u32) {
        self.lines.push(Line {
            start: self.code.len() - 1,
            line,
        })
    }

    pub fn write_chunk(&mut self, code: u8, line: u32) {
        self.code.push(code);

        match self.lines.last() {
            None => self.new_line(line),
            Some(last) if last.line != line => self.new_line(line),
            _ => (),
        };
    }

    pub fn write_constant(&mut self, value: Value, line: u32) {
        let len = self.constants.len();
        if len < 256 {
            self.write_chunk(OP_CONSTANT, line);
            self.write_chunk(len as u8, line);
        } else {
            let bytes = value.to_be_bytes();
            self.write_chunk(OP_CONSTANT, line);
            self.write_chunk(bytes[0], line);
            self.write_chunk(bytes[1], line);
            self.write_chunk(bytes[2], line);
            self.write_chunk(bytes[3], line);
        }

        self.constants.push(value)
    }
}
