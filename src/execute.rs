
use std::io::{stdin, Read, BufRead};

use crate::lexer::Token;

#[derive(Debug)]
pub struct Interpret {
    stackp: Vec<usize>,
    memory: Vec<u8>,
    tokens: Vec<Token>,
    instrp: usize,
    memp  : usize
}


impl Interpret {
    pub fn new(tokens: Vec<Token>) -> Interpret {
        Interpret {
            stackp: Vec::new(),
            memory: vec![0; 30_000],
            tokens,
            instrp: 0,
            memp  : 0
        }
    }

    ///  Used to execute the stream of tokens obtained by parsing the string of
    /// data provided to the interpreter
    pub fn execute(mut self) {
        loop {
            let token = self.
                tokens.
                get(self.instrp).
                unwrap();
            
            // On OBRACKETS, instead of pushin instrp + 1 to the stack, we push
            // instrp. That is because as we add the value of the instruction
            // pointer after the match statement, and this would mean that 
            // after restarting the execution of the loop, the effective
            // position for instrp would be instrp + 2, that is, its value
            // would be bigger than the desired value by 1
            match *token {
                Token::INCMEMPTR => self.memp = self.memp + 1,
                Token::DECMEMPTR => self.memp = self.memp - 1,
                Token::INCVAL    => self.inc_val_at_memp(),
                Token::DECVAl    => self.dec_val_at_memp(),
                Token::OBRACKETS => self.stackp.push(self.instrp),
                Token::CBRACKETS => self.stop_loop(),
                Token::GETVAl    => self.get_char(),
                Token::PUTVAL    => self.print_mem(), 
                _                => ()
            }

            self.instrp = self.instrp + 1;
            if self.instrp == self.tokens.len() {break};
        }
    }

    // TODO: handle error
    fn inc_val_at_memp(&mut self) {
        let val = self.
            memory.
            get_mut(self.memp).
            unwrap();

        *val = *val + 1;
    }

    // TODO: handle error
    fn dec_val_at_memp(&mut self) {
        let val = self.
            memory.
            get_mut(self.memp).
            unwrap();

        *val = *val - 1;
    }

    ///   Check wheter the value on memory pointed to by the memory pointer 
    /// is equal to 0. If the value is equal to 0, stop execution of the
    /// inner loop. Otherwise, restart execution at the first token after
    /// the brackets anouncing the start of the loop
    fn stop_loop(&mut self) {
        let memval = self.
            memory.
            get(self.memp).
            unwrap();

        if *memval == 0 {
            self.stackp.pop();
        } else {
            let loop_start = self.
                stackp.
                last().
                unwrap();

            self.instrp = *loop_start;
        }

    }

    /// Print value pointed to by the memory pointer
    fn print_mem(&self) {
        let character = self.
                    memory.
                    get(self.memp).
                    unwrap();

        print!("{}", *character as char);
    }



    fn get_char(&mut self) {
        let mut buffer: String = String::new();
        let _result = stdin().lock().read_line(&mut buffer).unwrap();
        ascii_raw_to_bytes(&mut buffer);

        let addrp = self.
            memory.
            get_mut(self.memp).
            unwrap();

        *addrp = *buffer.
            as_bytes().
            get(0).
            unwrap();
    }
}

fn ascii_raw_to_bytes(string: &mut String) {
    *string = string.replace("\\n", "\n");
    *string = string.replace("\\t", "\t");
    *string = string.replace("\\0", "\0");
}
