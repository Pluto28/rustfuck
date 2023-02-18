mod lexer;
mod execute;


use std::fs::File;
use crate::lexer::Tokenize;
use execute::Interpret;

fn main() {
    let string = String::from(">,[>,]<[<]>[.>]");
    let tokenized = string.to_tokens();
    println!("{:?}", tokenized);
    let interpreter = Interpret::new(tokenized);
    interpreter.execute();
}
