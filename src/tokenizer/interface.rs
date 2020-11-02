use tendril::StrTendril;
// use super:: {Reserved, Tokens};

// a trait for types that can be built from a python script 
pub trait PyEntity: std::fmt::Debug {
    // to add a docstring 
    fn add_helptext(&mut self, help_text: StrTendril);
    // to add a comment 
    fn add_comment(&mut self, comment: StrTendril);
    // to process some text 
    fn process_text(&mut self, text: StrTendril);
}


#[derive(Debug)]
pub struct Class {
    name: StrTendril,
    depth: u32,
    line: u32,
    is_subclass: bool,
    super_class: Option<StrTendril>,
    blocks: Vec<Box<dyn PyEntity>>,
    help_text: Option<StrTendril>,
    comment: Option<StrTendril>
}

impl PyEntity for Class {
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
        println!("Text on class {:?}", text);
    }
}

impl Class {
    pub fn new(depth: u32, line: u32) -> Self {
        Self {
            name: StrTendril::new(),
            depth,
            line,
            is_subclass: false,
            super_class: None, 
            blocks: vec![],
            help_text: None,
            comment: None 
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

    // process a label 
    pub fn process_label(&mut self, label: StrTendril){
        // if the class has name, then it's probably a superclass name 
        if self.has_name(){
            // if is_sublass name is set and no superclass name, set super class 
            if self.is_subclass && self.super_class.is_none(){
                self.set_superclass(label);
            }
        }else{
            // we should set the class name 
            self.set_name(label);
        }
    }

}