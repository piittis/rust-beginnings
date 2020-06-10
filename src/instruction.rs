// Operation codes
pub const OP_RETURN: u8 = 1;
pub const OP_CONSTANT: u8 = 2;
pub const OP_CONSTANT_LONG: u8 = 3;
pub const OP_ADD: u8 = 4;
pub const OP_SUBTRACT: u8 = 5;
pub const OP_MULTIPLY: u8 = 6;
pub const OP_DIVIDE: u8 = 7;
pub const OP_NEGATE: u8 = 8;

pub type Value = f64;

pub enum Instruction {
    OpReturn,
    OpConstant(u8),
    OpConstantLong(u32),
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNegate,
}

// Decodes an instruction starting at index ip, returns the instruction and its size in bytes.
pub fn decode(code: &Vec<u8>, ip: usize) -> (usize, Instruction) {
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
