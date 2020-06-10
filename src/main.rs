mod chunk;
mod debug;
mod instruction;
mod vm;

use crate::instruction::OP_ADD;
use crate::instruction::OP_DIVIDE;
use crate::instruction::OP_NEGATE;
use crate::instruction::OP_RETURN;
use chunk::Chunk;
use vm::VM;

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
