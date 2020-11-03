use super::{PyEntity, EntityType, Reserved};
use tendril::StrTendril;

#[derive(Debug)]
pub struct Function{
    name: StrTendril,
    num_of_args: u32,
    args: Vec<StrTendril>,
    line_no: u32,
    depth: u32,
    context: Option<u32>,
    blocks: Vec<Box<dyn PyEntity>>,
    processing_args: bool,
    help_text: StrTendril,
    comment: StrTendril
}

impl PyEntity for Function{
    // get the type 
    fn get_type(&self) -> EntityType {
        EntityType::new("function")
    }

    fn get_depth(&self) -> u32 {
        self.depth
    }

    fn get_line(&self) -> u32 {
        self.line_no
    }

    // add a docstring 
    fn add_helptext(&mut self, help_text: StrTendril){
        self.help_text = help_text;
    }

    fn add_comment(&mut self,comment: StrTendril){
        self.comment = comment;
    }

    // add a block 
    fn add_block(&mut self, block: Box<dyn PyEntity>){
        self.blocks.push(block);
    }

    // process a text label
    fn process_text(&mut self, text: StrTendril){
        let arg = Reserved::from_tendril(&text);

        match arg {
            Reserved::This => {
                // assert that there is a context available 
                assert!(
                    self.context.is_some()
                ) 
            },
            Reserved::Label(c) => {
                // add an argument
                if self.name.len32() == 0 {
                    // add a function name 
                    self.name = c;
                } else {
                    if self.processing_args{
                        self.num_of_args += 1;
                        self.args.push(c);
                    }
                }
                
            },
            Reserved::Init => {
                // constructor is the name 
                self.name = StrTendril::from_slice("constructor");
            }
            _ => ()
        }
        
    }
}

impl Function {
    // create a new instance 
    pub fn new(depth: u32, line_no: u32, context: Option<u32>) -> Self {
        Self {
            name: StrTendril::new(),
            num_of_args: 0,
            args: vec![],
            line_no,
            depth,
            context,
            blocks: vec![],
            processing_args: false,
            help_text: StrTendril::new(),
            comment: StrTendril::new()
        }

    }

    // toggle processing args 
    pub fn toggle_args(&mut self) {
        self.processing_args = !self.processing_args;
    }

}