use crate::parser::Parse;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    OBRACKETS(usize),
    CBRACKETS(usize),
    INCMEMPTR,
    DECMEMPTR,
    INCVAL,
    DECVAl,
    GETVAl,
    PUTVAL,
    COMMENT,
}

pub trait Tokenize {
    fn to_tokens(&self) -> Vec<Token>;
}

impl Tokenize for String {
    fn to_tokens(&self) -> Vec<Token> {
        let chars = self.chars();
        let mut tokens: Vec<Token> = Vec::new();
        let mut tindex = 0;

        for ch in chars {
            let token = match ch {
                '[' => Token::OBRACKETS(0),
                ']' => Token::CBRACKETS(0),
                '>' => Token::INCMEMPTR,
                '<' => Token::DECMEMPTR,
                '+' => Token::INCVAL,
                '-' => Token::DECVAl,
                ',' => Token::GETVAl,
                '.' => Token::PUTVAL,
                _ => Token::COMMENT,
            };

            //if token == Token::OBRACKETS {
            //    print!(" ({}: {}  {:?}) ", ch, tindex, token);
            //}

            if token != Token::COMMENT {
                //print!(" ({}: {}  {:?}) ", ch, index, token);
                tindex = tindex + 1;
                tokens.push(token);
            }
        }

        tokens.parse();
        tokens
    }
}

impl Parse for Vec<Token> {
    fn parse(&mut self) {}

    ///   Lookup recursively, finding the starting and ending index
    /// of every OBRACKETS or CBRACKETS token, replacing the unfiled
    /// portion of the enum variant with the value on the token stream
    /// it sits
    fn lookup(&mut self, openi: usize) -> usize {
        let mut tokeni = openi;
        let mut matchi = openi;

        for token in self[openi..].iter_mut() {
            match *token {
                Token::OBRACKETS(val) => {
                    self.lookup(tokeni);

                }
                Token::CBRACKETS(val) => {
                    matchi = tokeni;

                }
                _ => (),
            }

            tokeni = tokeni + 1;
        }

        matchi
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obrackets() {
        let data = String::from("[").to_tokens().pop().unwrap();
        assert_eq!(data, Token::OBRACKETS(_));
    }

    #[test]
    fn test_cbrackets() {
        let data = String::from("]").to_tokens().pop().unwrap();
        assert_eq!(data, Token::CBRACKETS(_));
    }

    #[test]
    fn test_incmemptr() {
        let data = String::from(">").to_tokens().pop().unwrap();
        assert_eq!(data, Token::INCMEMPTR);
    }

    #[test]
    fn test_decmemptr() {
        let data = String::from("<").to_tokens().pop().unwrap();
        assert_eq!(data, Token::DECMEMPTR);
    }

    #[test]
    fn test_incval() {
        let data = String::from("+").to_tokens().pop().unwrap();
        assert_eq!(data, Token::INCVAL);
    }

    #[test]
    fn test_decval() {
        let data = String::from("-").to_tokens().pop().unwrap();
        assert_eq!(data, Token::DECVAl);
    }

    #[test]
    fn test_getval() {
        let data = String::from(",").to_tokens().pop().unwrap();
        assert_eq!(data, Token::GETVAl);
    }

    #[test]
    fn test_putval() {
        let data = String::from(".").to_tokens().pop().unwrap();
        assert_eq!(data, Token::PUTVAL);
    }
}
