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
                '[' => Token::OBRACKETS(tindex),
                ']' => Token::CBRACKETS(tindex),
                '>' => Token::INCMEMPTR,
                '<' => Token::DECMEMPTR,
                '+' => Token::INCVAL,
                '-' => Token::DECVAl,
                ',' => Token::GETVAl,
                '.' => Token::PUTVAL,
                _ => Token::COMMENT,
            };

            if token != Token::COMMENT {
                print!(" ({}: {}  {:?}) ", ch, tindex, token);
                tindex = tindex + 1;
                tokens.push(token);
            }
        }

        tokens.update_matches();
        println!("\n\n\n\n{:?}", tokens);
        tokens
    }
}

impl Parse for Vec<Token> {
    ///     Lookup matching OBRACKETS and CBRACKETS tokens and update the value
    /// held by such variants
    fn update_matches(&mut self) {
        let mut recurse_val = 0;
        let mut ob_stack: Vec<usize> = Vec::new();
        let mut tokeni = 0;

        loop {
            let token = self.get_mut(tokeni).unwrap();

            match *token {
                Token::OBRACKETS(_) => {
                    recurse_val = recurse_val + 1;
                    ob_stack.push(tokeni);
                }

                Token::CBRACKETS(_) => {
                    recurse_val = recurse_val - 1;
                    let ob_i = ob_stack.pop().unwrap();

                    *token = Token::CBRACKETS(ob_i);

                    let obracket = self.get_mut(ob_i).unwrap();
                    *obracket = Token::OBRACKETS(tokeni);
                }

                _ => (),
            }

            tokeni = tokeni + 1;

            if tokeni == self.len() { break }
        }
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
