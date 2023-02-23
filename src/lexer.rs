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

            if token != Token::COMMENT {
                tokens.push(token);
            }
        }

        tokens.update_matches();
        // debug. Maybe later implement it as a trait
        //println!("\n\n\n\n{:?}", tokens);
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obrackets() {
        let data = String::from("[]").to_tokens().pop().unwrap();
        assert_eq!(data, Token::CBRACKETS(0));
    }

    #[test]
    fn test_cbrackets() {
        let data = String::from("[]").to_tokens();
        let token = data.get(0).unwrap();
        assert_eq!(*token, Token::OBRACKETS(1));
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
