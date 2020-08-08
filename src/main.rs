mod chunk;
mod compiler;
mod debug;
mod instruction;
mod scanner;
mod vm;

use chunk::Chunk;
use compiler::compile;
use std::env;
use std::fs;
use std::io;
use std::io::Write;
use vm::VM;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => repl(),
        2 => run_file(),
        _ => {} // report error
    }
}

fn repl() {
    loop {
        print!("> ");
        io::stdout().flush();
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("cant read line");

        interpret(&line);
    }
}

fn run_file() {
    let file = read_file();

    interpret(&file);
    println!("runFile");
}

fn read_file() -> String {
    fs::read_to_string("./foo").unwrap()
}

fn interpret(source: &str) {
    let chunk = Chunk::new();
    compile(source, &chunk);
    VM::interpret(&chunk);
}
