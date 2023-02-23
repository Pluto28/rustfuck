use std::io::{stdin, BufRead};

use crate::lexer::Token;

#[derive(Debug)]
pub struct Interpret {
    memory: Vec<u8>,
    tokens: Vec<Token>,
    instrp: usize,
    memp: usize,
}

impl Interpret {
    pub fn new(tokens: Vec<Token>) -> Interpret {
        Interpret {
            memory: vec![0; 30_001],
            tokens,
            instrp: 0,
            memp: 0,
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
                    let memv = *self.memory.
                        get(self.memp).
                        unwrap();

                    if memv == 0 {
                        self.instrp = closemb;
                    }
                }
                Token::CBRACKETS(openmb) => {
                    let memv = *self.memory.
                        get(self.memp).
                        unwrap();

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

    // TODO: handle error
    fn inc_val_at_memp(&mut self) {
        let val = self.memory.get_mut(self.memp).unwrap();

        *val = val.wrapping_add(1);
    }

    // TODO: handle error
    fn dec_val_at_memp(&mut self) {
        let val = self.memory.get_mut(self.memp).unwrap();

        *val = val.wrapping_sub(1);
    }


    /// Print value pointed to by the memory pointer
    fn print_mem(&self) {
        let character = self.memory.get(self.memp).unwrap();

        print!("{}", *character as u8 as char);
    }

    fn get_char(&mut self) {
        let mut buffer: String = String::new();
        let _result = stdin().lock().read_line(&mut buffer).unwrap();
        ascii_raw_to_bytes(&mut buffer);

        let addrp = self.memory.get_mut(self.memp).unwrap();

        *addrp = *buffer.as_bytes().get(0).unwrap();
    }
}

fn ascii_raw_to_bytes(string: &mut String) {
    *string = string.replace("\\n", "\n");
    *string = string.replace("\\t", "\t");
    *string = string.replace("\\0", "\0");
}
