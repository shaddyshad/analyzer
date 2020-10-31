use super::{Tokens, Reserved, Stack, Class, PyEntity};
use tendril::StrTendril;


/// Token sink 
#[derive(Debug)]
pub struct TokenSink{
    subsequent_quotes: u32,
    docstring_mode: bool, 
    docstring: StrTendril,
    subsequent_spaces: u32,
    depth: u32,             // track the hierachy depth
    docstring_stack: Stack<StrTendril>,
    depth_stack: Stack<u32> ,
    line_number: u32,
    current_class: Vec<Box<dyn PyEntity>>

}

impl TokenSink{
    pub fn new() -> Self {
        Self {
            subsequent_quotes: 0,
            docstring_mode: false,
            docstring: StrTendril:: new(),
            subsequent_spaces: 0,
            depth: 0,
            docstring_stack: Stack::new(),
            depth_stack: Stack::new(),
            line_number: 0,
            current_class: vec![]
        }
    }

    // process a token 
    pub fn process(&mut self, token: Tokens, line_no: u32){
        self.pre_process(&token, line_no);

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
                    let res = Reserved::from_tendril(&tok);
                    if let Reserved::Class = res{
                        self.create_class();
                    }else{
                        // check if we have any processing class 
                        if let Some(ref mut cls) = self.current_class.last_mut() {
                            cls.process(res);
                        }
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
            _ => ()
        }

    }

    fn commit_docstring(&mut self){
        // commit a docstring
        self.docstring_stack.push(self.docstring.clone()); 
        self.docstring.clear()
    }



    fn pre_process(&mut self, token: &Tokens, line_no: u32){
        // pre process for cases of docstring and subsequent counts\
        match token {
            Tokens::StringSingle | Tokens::StringDouble => {
                self.subsequent_quotes += 1;        // increase number of subsequent quotes 
                self.subsequent_spaces= 0;
            },
            Tokens::Space => {
                let spaces = self.subsequent_spaces +1;

                // if four subsequent spaces add 1 to depth 
                if spaces == 4 {
                    self.depth += 1;
                    self.subsequent_spaces = 0;
                }else{
                    self.subsequent_spaces = spaces;
                }
            },
            _ => {
                self.subsequent_quotes = 0;
                self.subsequent_spaces= 0;
            }
        }

        // adjust new lines 
        self.adjust_depth_on_newline(line_no);

    }

    /// each new line should reset the line number 
    fn adjust_depth_on_newline(&mut self, line_no: u32){
        if line_no != self.line_number{
            // commit the depth 
            self.depth_stack.push(self.depth);

            // changing line number 
            self.depth = 0;
            self.subsequent_spaces = 0;
            self.line_number = line_no;
        }
    }

    // create a new class 
    fn create_class(&mut self){
        let class = Class::new(self.line_number, self.depth);

        self.current_class.push(Box::new(class));
    }

}