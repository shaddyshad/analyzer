use tendril::StrTendril;
use super:: {Reserved, Tokens};

// a trait for types that can be built from a python script 
pub trait PyEntity: std::fmt::Debug {
    fn process(&mut self, token: Reserved, line_no: u32, depth: Option<&u32>);
    fn process_token(&mut self, token: Tokens, line_no: u32, depth: Option<&u32>);
}

#[derive(Debug)]
pub struct Class {
    pub start_line: u32,
    pub last_line: u32,
    pub depth: u32,
    pub name: StrTendril,
    pub is_subclass: bool,
    pub super_class: Option<StrTendril>,
    processing_definition: bool,
    commited: bool
}

impl PyEntity for Class {
    fn process(&mut self, token: Reserved, line_no: u32, depth: Option<&u32>){
        self.check_last_line(line_no, depth);

        if line_no == self.start_line {
            // class definition line 
            match token {
                Reserved::Label(c) => {
                    // check if class has a class name yet 
                    if self.name.len32() == 0 {
                        self.name = c;
                    }else{
                        self.process_class_definition(c);
                    }
                },
                _ => {
    
                }
            }
        }else{
            self.processing_definition = false;
        }
        
    }

    fn process_token(&mut self, token: Tokens, line_no: u32, depth: Option<&u32>){
        self.check_last_line(line_no, depth);
        // process a token 
        match token {
            Tokens::OpeningPar => {
                // if we are in the first line, it is a super class definition 
                if line_no == self.start_line {
                    self.is_subclass = true;
                    self.processing_definition = true;
                }else{
                    self.processing_definition = false;
                }
            },
            _ => {}
        }
    }
}

impl Class {
    pub fn new(start: u32, depth: u32) -> Self {
        Self {
            start_line: start,
            depth,
            last_line: 0,
            is_subclass: false,
            super_class: None,
            name: StrTendril::new(),
            processing_definition: false,
            commited: false 
        }
    }

    fn process_class_definition(&mut self, def: StrTendril){
        // if we are processing a sub class 
        if self.processing_definition {
            if self.is_subclass & self.super_class.is_none() {
                self.super_class = Some(def);
            }
        }
    }

    fn check_last_line(&mut self, line_no: u32, depth: Option<&u32>){
        // if depth changes 
        if let Some(d) = depth {
            if d == &self.depth && !self.commited && line_no != self.start_line && !self.processing_definition{
                

                self.last_line = line_no;
                self.commited = true;
            }
        }
        
    }
}