use super::{Tokens, Stack, PyEntity, Line, LineTypes, Class};



/// Token sink 
#[derive(Debug)]
pub struct TokenSink{
    builders: Stack<Box<dyn PyEntity>>
}

impl TokenSink{
    pub fn new() -> Self {
        Self { 
            builders: Stack::new()
        }
    }

    // process a token 
    pub fn process(&mut self, line: Line){
        let line_type = line.line_type();

        // process the tokens 
        match line_type {
            LineTypes::ClassDeclaration => {
                self.process_class_declaration(line);
            }
            _ => ()
        }
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
                    class.commit_superclass();
                },
                _ => ()
            }
        }

        // add the class to builders 
        self.builders.push(Box::new(class));

    }

}