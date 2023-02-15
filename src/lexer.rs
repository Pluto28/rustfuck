use std::fs::File;

#[derive(Debug)]
pub enum Token {
    OBRACKETS,
    CBRACKETS,
    INCMEMPTR,
    DECMEMPTR,
    INCVAL,
    DECVAl,
    GETVAl,
    PUTVAL,
    COMMENT
}

pub trait Tokenize {
    fn to_tokens(&self) -> Vec<Token>;
}

impl Tokenize for String {
    fn to_tokens(&self) -> Vec<Token> {
        let chars = self.chars();
        let mut tokens: Vec<Token> = Vec::new();

        for ch in chars {
            let token = match ch {
                '[' => Token::OBRACKETS,
                ']' => Token::CBRACKETS,
                '>' => Token::INCMEMPTR,
                '<' => Token::DECMEMPTR,
                '+' => Token::INCVAL,
                '-' => Token::DECVAl,
                ',' => Token::GETVAl,
                '.' => Token::PUTVAL,
                 _  => Token::COMMENT
            };

            tokens.push(token);
        }

        tokens
    }
}
