mod execute;
mod lexer;
mod parser;

use crate::lexer::Tokenize;
use execute::Interpret;
use parser::Parse;
use std::fs::File;
use std::env;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let filename = args.
            get(1).
            unwrap().
            as_str();

        let mut filep = File::open(filename).expect("[Error]: Opening file");
        let mut buf = String::new();


        filep.read_to_string(&mut buf).expect("[Error]: Reading file");
        execute_interpreter(buf);
    } else {
        println!("usage: rustfuck <file.bf>");
    }
}

fn execute_interpreter(program: String) {
    let token_stream = program.to_tokens();
    let interpreter = Interpret::new(token_stream);
    interpreter.execute();
}
