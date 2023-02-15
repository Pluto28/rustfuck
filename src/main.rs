mod lexer;
use std::fs::File;
use crate::lexer::Tokenize;

fn main() {
    let string = String::from("[>>>]");

    let tokenized = string.to_tokens();
    println!("{:?}", tokenized);
}
