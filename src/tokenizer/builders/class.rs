use super::{PyEntity, EntityType};
use std::collections::HashMap;
use tendril::StrTendril;

#[derive(Debug)]
pub struct Class {
    name: StrTendril,
    depth: u32,
    line_no: u32,
    is_subclass: bool,
    super_class: Option<StrTendril>,
    blocks: Vec<Box<dyn PyEntity>>,
    help_text: Option<StrTendril>,
    comment: Option<StrTendril>,
    attributes: HashMap<StrTendril, StrTendril>
}

impl PyEntity for Class {
    // get the type 
    fn get_type(&self) -> EntityType {
        EntityType::new("class")
    }

    fn get_depth(&self) -> u32 {
        self.depth
    }

    fn get_line(&self) -> u32 {
        self.line_no
    }

    // add a comment to an entity 
    fn add_comment(&mut self, comment: StrTendril) {
        self.comment = Some(comment);
    }

    // add help text (docstring)
    fn add_helptext(&mut self, help_text: StrTendril){
        self.help_text = Some(help_text);
    }

    // process some other text 
    fn process_text(&mut self, text: StrTendril){
        // if the class has name, then it's probably a superclass name 
        if self.has_name(){
            // if is_sublass name is set and no superclass name, set super class 
            if self.is_subclass && self.super_class.is_none(){
                self.set_superclass(text);
            }
        }else{
            // we should set the class name 
            self.set_name(text);
        }
    }

    // add a block 
    fn add_block(&mut self, block: Box<dyn PyEntity>){
        self.blocks.push(block);
    }
}

impl Class {
    pub fn new(depth: u32, line_no: u32) -> Self {
        Self {
            name: StrTendril::new(),
            depth,
            line_no,
            is_subclass: false,
            super_class: None, 
            blocks: vec![],
            help_text: None,
            comment: None,
            attributes: HashMap::new()
        }
    }

    fn set_name(&mut self, name: StrTendril){
        self.name = name;
    }

    fn set_superclass(&mut self, super_class: StrTendril){
        self.super_class = Some(super_class);
        self.is_subclass = true;
    }

    pub fn set_subclass(&mut self, subclass: bool){
        self.is_subclass = subclass;
    }

    pub fn has_name(&self) -> bool {
        self.name.len32() != 0
    }

    /// commit a superclass 
    /// find the definition of the super class 
    pub fn commit_superclass(&mut self){
        println!("Super class defined");
    }


}