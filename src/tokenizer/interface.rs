use tendril::StrTendril;
use super:: Reserved;

// a trait for types that can be built from a python script 
pub trait PyEntity: std::fmt::Debug {
    fn process(&mut self, token: Reserved);
}

#[derive(Debug)]
pub struct Class {
    pub start_line: u32,
    pub last_line: u32,
    pub depth: u32,
    pub name: StrTendril,
    pub is_subclass: bool,
    pub super_class: Option<StrTendril>
}

impl PyEntity for Class {
    fn process(&mut self, token: Reserved){
        match token {
            Reserved::Label(c) => {
                // check if class has a class name yet 
                if self.name.len32() == 0 {
                    self.name = c;
                }
            },
            _ => {

            }
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
            name: StrTendril::new()
        }
    }
}