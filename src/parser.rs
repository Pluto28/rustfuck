// lookup the token stream for matching open and closing brackets. This should make 
// our life easier at the execution step

use crate::lexer::Token;

pub trait Parse {
    fn update_matches(&mut self);

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
