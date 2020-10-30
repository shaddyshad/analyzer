use super::{Tokens, Reserved};
use tendril::StrTendril;


/// Token sink 
#[derive(Debug)]
pub struct TokenSink {
    subsequent_quotes: u32,
    docstring_mode: bool, 
    docstring: StrTendril,
    subsequent_spaces: u32,
    depth: u32,             // track the hierachy depth 
}

impl TokenSink {
    pub fn new() -> Self {
        Self {
            subsequent_quotes: 0,
            docstring_mode: false,
            docstring: StrTendril:: new(),
            subsequent_spaces: 0,
            depth: 0
        }
    }

    // process a token 
    pub fn process(&mut self, token: Tokens){
        self.pre_process(&token);

        match token {
            Tokens::Token(tok) => {
                // check if it is a docstring 
                if self.docstring_mode {
                    // append tok to docstring 
                    // add a space 
                    self.docstring.push_slice(&tok);
                    self.docstring.push_slice(" ");
                }else{
                    // process a single token, a token could contain a reserved keyword or a definition
                    match Reserved::from_tendril(&tok){
                        Reserved::Label(c) => (),
                        Reserved::Class => {
                            // finalize any pending processing 
                            // and create a new class processor
                            
                        },
                        _ => ()
                    }
                }
                
            },
            Tokens::StringDouble | Tokens::StringSingle => {
                // handle the case of a doc string 
                let count = self.subsequent_quotes ;

                if count == 3 {
                    //toggle docstring mode or / off 
                    if self.docstring_mode {
                        // commit a doctstring 
                        self.docstring_mode = false;
                        self.commit_docstring();
                    }else{
                        self.docstring_mode = true;
                    }

                    self.subsequent_quotes = 0;
                }else if count > 0{
                    // could build a doctring 
                    self.subsequent_quotes = count;
                }
            },
            Tokens::Space => {
                // check for depth change 
                let c = self.subsequent_spaces / 4;


                if c >= 1{
                    // increase the level 
                    self.commit_depth(c);
                }
            }
            _ => ()
        }
    }

    fn commit_docstring(&mut self){
        // commit a docstring 
        self.docstring.clear()
    }

    fn commit_depth(&mut self, depth: u32){
        // commit a depth 
        self.depth = depth;
        
    }

    fn pre_process(&mut self, token: &Tokens){
        // pre process for cases of docstring and subsequent counts\
        match token {
            Tokens::StringSingle | Tokens::StringDouble => {
                self.subsequent_quotes += 1;        // increase number of subsequent quotes 
                self.subsequent_spaces = 0;         // reset subsequent spaces
                
            },
            Tokens::Space => {
                self.subsequent_quotes = 0;
                self.subsequent_spaces += 1;
            },
            _ => {
                self.subsequent_quotes = 0;
                self.subsequent_spaces = 0;
            }
        }
    }

}