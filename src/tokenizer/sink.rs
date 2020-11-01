use super::{Tokens, Reserved, Stack, Class, PyEntity, Line};
use tendril::StrTendril;


/// Token sink 
#[derive(Debug)]
pub struct TokenSink{}

impl TokenSink{
    pub fn new() -> Self {
        Self { }
    }

    // process a token 
    pub fn process(&mut self, line: Line){
        println!("{:#?}", line.depth());
    }

}