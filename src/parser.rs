// lookup the token stream for matching open and closing brackets. This should make 
// our life easier at the execution step

use crate::lexer::Token;

pub trait Parse {
    fn parse(&mut self);
    fn lookup(&mut self, tokeni: usize) -> usize;

}
