use tendril::StrTendril;
// use super:: {Reserved, Tokens};

// types that can be returned 
#[derive(Debug, Eq, PartialEq)]
pub struct EntityType( pub StrTendril );

impl EntityType {
    pub fn new(arg: &'static str) -> Self {
        Self (StrTendril::from_slice(arg))
    }

    fn is(&self, arg: &'static str) -> bool {
        self.0 == StrTendril::from_slice(arg)
    }

    // check if it is a class 
    pub fn is_class(&self) -> bool {
        self.is("class")
    }
}

// a trait for types that can be built from a python script 
pub trait PyEntity: std::fmt::Debug{
    // to add a docstring 
    fn add_helptext(&mut self, help_text: StrTendril) {}
    // to add a comment 
    fn add_comment(&mut self, comment: StrTendril) {}
    // to process some text 
    fn process_text(&mut self, text: StrTendril) {}

    // to add a block 
    fn add_block(&mut self, block: Box<dyn PyEntity>) {}

    // get the type 
    fn get_type(&self) -> EntityType;

    // get the line number 
    fn get_line(&self) -> u32;
    // get the depth 
    fn get_depth(&self) -> u32;
}

