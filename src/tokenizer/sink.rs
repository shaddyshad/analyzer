use super::{Tokens, Stack, PyEntity, Line, LineTypes, Class, Function};
use tendril::StrTendril;


/// Token sink 
#[derive(Debug)]
pub struct TokenSink{
    builders: Stack<Box<dyn PyEntity>>,
    processing_docstring: bool
}

impl TokenSink{
    pub fn new() -> Self {
        Self { 
            builders: Stack::new(),
            processing_docstring: false
        }
    }

    // process a token 
    pub fn process(&mut self, line: Line){
        let line_type = line.line_type();

        // process the tokens 
        match line_type {
            LineTypes::ClassDeclaration => {
                self.create_class(line);
            },
            LineTypes::DocString => {
                // toggle the docstring processing mode
                self.processing_docstring = !self.processing_docstring;
            },
            LineTypes::Text(_) => {
                // collect the text from tokens 
                let text = self.get_text(&line).expect("No text tokens in this line");

                // if we are in docstring mode, set the docstring of the itemin the top 
                if let Some(ref mut builder) = self.builders.top_mut(){
                    // use the builder to add a docstring 
                    if self.processing_docstring {
                        builder.add_helptext(text);
                    }else{
                        // some other text 
                        builder.process_text(text);
                    }
                    
                }
                
            },
            LineTypes::FunctionCreation => {
                // create a new function 
                self.create_function(line);
            }
            _ => ()
        }
    }

    // get some text from a line 
    fn get_text(&self, line: &Line) -> Option<StrTendril>{
        let tokens = line.trim();

        // loop through finding text tokens 
        let mut text = StrTendril::new();

        for token in tokens {
            if let Tokens::Token(c) = token {
                // push the slice 
                text.push_tendril(&c);
                text.push_slice(" ");
            }
        }

        Some(text)
    }

    // create a new function 
    fn create_function(&mut self, line: Line){
        // trim the first token 
        let tokens = self.trim_first_token(&line);

        // find the context of this 
        let context = self.get_context();

        // create a new function 
        let mut function = Function::new(line.depth(), line.line_number, context);

        // iterate the tokens filling in the function 
        for token in tokens {
            match token {
                Tokens::Token(c) => {
                    // process a label 
                    function.process_text(c.clone())
                },
                Tokens::OpeningPar => function.toggle_args(),
                Tokens::ClosingPar => function.toggle_args(),
                _ => ()
            }
        }

        // add to the builders 
        self.builders.push(Box::new(function));
    }

    // get the function context 
    fn get_context(&self) -> Option<u32>{
        if let Some(ref builder) = self.builders.top(){
            // check the type of builder
            let b_type = builder.get_type();

            if b_type.is_class(){
                // clone the reference 
                Some(builder.get_line())

            }else{
                None
            }

        }else{
            None 
        }
    }

    // trim and remove the first token from a line 
    fn trim_first_token(&self, line: &Line) -> Vec<Tokens>{
        let tokens = line.trim();

        let tokens = &tokens[1..];

        tokens.to_vec()
    }

    // process a class declaration 
    fn create_class(&mut self, line: Line){
        // remove the first token because it's 'class' and we already know we are building 
        // a new class 
        let tokens = self.trim_first_token(&line);
        let mut class = Class::new(line.depth(), line.line_number);

        // process them in turn 
        for token in tokens {
            match token {
                Tokens::Token(c) => {
                    // process a label 
                    class.process_text(c.clone());
                },
                Tokens::OpeningPar => {
                    // a '(' in clas declaration indicates a sub class definition 
                    class.set_subclass(true);
                },
                Tokens::ClosingPar => {
                    // can be used to indicate completion of a superclass definition 
                    // find the definition of the super cclass and store it 
                    class.commit_superclass();
                },
                _ => ()
            }
        }

        // add the class to builders 
        self.builders.push(Box::new(class));

    }

}