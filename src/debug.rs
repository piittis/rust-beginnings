use crate::chunk::Chunk;
use crate::instruction::decode;
use crate::instruction::Instruction;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut ip = 0;
    let len = chunk.code.len();

    while ip < len {
        ip += disassemble_instruction(&chunk, ip);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, ip: usize) -> usize {
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
