use std::fs::File;

#[derive(Debug, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obrackets() {
        let data = String::from("[").to_tokens().pop().unwrap();
        assert_eq!(data, Token::OBRACKETS);
    }

    #[test]
    fn test_cbrackets() {
        let data = String::from("]").to_tokens().pop().unwrap();
        assert_eq!(data, Token::CBRACKETS);
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
