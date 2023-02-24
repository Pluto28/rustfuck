use std::io::{stdin, stdout, BufRead, Write};

use crate::lexer::Token;

#[derive(Debug)]
pub struct Interpret {
    memory: Vec<u64>,
    tokens: Vec<Token>,
    instrp: usize,
    memp: usize,
    charb: Vec<char>,
}

impl Interpret {
    pub fn new(tokens: Vec<Token>) -> Interpret {
        Interpret {
            memory: vec![0; 30_001],
            tokens,
            instrp: 0,
            memp: 0,
            charb: Vec::new(),
        }
    }

    ///  Used to execute the stream of tokens obtained by parsing the string of
    /// data provided to the interpreter
    pub fn execute(mut self) {
        loop {
            let token = self.tokens.get(self.instrp).unwrap();

            // On OBRACKETS, instead of pushin instrp + 1 to the stack, we push
            // instrp. That is because as we add the value of the instruction
            // pointer after the match statement, and this would mean that
            // after restarting the execution of the loop, the effective
            // position for instrp would be instrp + 2, that is, its value
            // would be bigger than the desired value by 1
            match *token {
                Token::INCMEMPTR => self.memp = self.memp + 1,
                Token::DECMEMPTR => self.memp = self.memp - 1,
                Token::INCVAL => self.inc_val_at_memp(),
                Token::DECVAl => self.dec_val_at_memp(),
                Token::OBRACKETS(closemb) => {
                    let memv = *self.memory.get(self.memp).unwrap();

                    if memv == 0 {
                        self.instrp = closemb;
                    }
                }
                Token::CBRACKETS(openmb) => {
                    let memv = *self.memory.get(self.memp).unwrap();

                    if memv != 0 {
                        self.instrp = openmb;
                    }
                },
                Token::GETVAl => self.get_char(),
                Token::PUTVAL => self.print_mem(),
                _ => (),
            }

            self.instrp = self.instrp + 1;
            if self.instrp >= self.tokens.len() {
                break;
            };
        }
    }

    fn inc_val_at_memp(&mut self) {
        let val = self.memory.get_mut(self.memp).unwrap();

        *val = val.wrapping_add(1);
    }

    fn dec_val_at_memp(&mut self) {
        let val = self.memory.get_mut(self.memp).unwrap();

        *val = val.wrapping_sub(1);
    }

    /// Print value pointed to by the memory pointer
    fn print_mem(&self) {
        let character = self.memory.get(self.memp).unwrap();
        print!("{}", *character as u8 as char);

        //  Flush stdout before printing anything. This avoid weird
        // printing behavior on the terminal
        stdout().flush().unwrap();
    }

    fn get_char(&mut self) {
        if self.charb.len() == 0 {
            let mut buffer = String::new();
            let _bytes = stdin().lock().read_line(&mut buffer).unwrap();

            let charb = buffer.chars().rev().collect();
            self.charb = charb;
        }

        if self.charb.len() > 0 {
            let addrp = self.memory.get_mut(self.memp).unwrap();
            *addrp = self.charb.pop().unwrap() as u64;
        }
    }
}

