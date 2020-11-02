use super::{Tokens, Stack, PyEntity, Line, LineTypes, Class};
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
                self.process_class_declaration(line);
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

    // process a class declaration 
    fn process_class_declaration(&mut self, line: Line){
        let tokens = line.trim();

        // remove the first token because it's 'class' and we already know we are building 
        // a new class 
        let tokens = &tokens[1..];
        let mut class = Class::new(line.depth(), line.line_number);

        // process them in turn 
        for token in tokens {
            match token {
                Tokens::Token(c) => {
                    // process a label 
                    class.process_label(c.clone());
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