use super::Tokens;

/// Token sink 
pub struct TokenSink {}

impl TokenSink {
    pub fn new() -> Self {
        Self {}
    }

    // process a token 
    pub fn process(&mut self, token: Tokens){
        println!("Token {:?}", token);
    }
}