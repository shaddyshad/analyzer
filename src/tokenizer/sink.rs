use super::{Tokens, Stack, PyEntity, Line, LineTypes, Class, Function, assignment, Reserved};
use tendril::StrTendril;
use assignment::{Assignment, Values, Variable, FunctionCall, ObjProp};


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
            },
            LineTypes::Assignment => {
                self.process_assignment(line);
            }
            _ => ()
        }
    }

    // process an assignment line
    fn process_assignment(&mut self, line: Line) {
        // trim 
        let tokens = line.trim();

        // create a new assignment block
        let mut block = Assignment::new(line.line_number, line.depth());

        // get the index of the equals sign 
        let index = tokens.iter().position(|r| r == &Tokens::Equals);


        // split the left side and right side by equals sign 
        if let Some(index) = index {
            // slice the rhs and lhs 
            let lhs = &tokens[0..index];
            let rhs = &tokens[index + 1..];

            //process the lhs 
            // check if lhs has a dot assignment which means it's an 
            // assignment to an object or self
            let dot_index = lhs.iter().position(|t| t == &Tokens::Dot);

            // split the object and the property being assigned 
            if let Some(dot_index) = dot_index {
                // process an object or context assignment 
                let obj = &lhs[0..dot_index];
                let prop = &lhs[dot_index+1..];

                // process each 
                if obj.len() > 0 && prop.len() > 0 {
                    // first item in slice 
                    let obj_name = &obj[0];
                    let prop_name = &prop[0];

                    // process the obj name 
                    if let Tokens::Token(c) = obj_name {
                        let res_obj = Reserved::from_tendril(c);

                        match res_obj {
                            Reserved::This => {
                                // get the prop name 
                                if let Tokens::Token(c) = prop_name {
                                    block.set_variable(Variable::Context(c.clone()));
                                }
                            },
                            Reserved::Label(c) => {
                                if let Tokens::Token(t) = prop_name {
                                    block.set_variable(Variable::Object(c.clone(), t.clone()));
                                }
                            }
                            _ => ()
                        }
                    }
                }
            }else{
                let variable = &lhs[0];

                if let Tokens::Token(c) = variable {
                    block.set_variable(Variable::Local(c.clone()))
                }
            }

            // process the right hand side 
            // find a dot which might be an object method call or property 
            let dot_index = rhs.iter().position(|t| t == &Tokens::Dot);


            if let Some(dot_index) = dot_index {
                // process the method assignment 
                let obj = &rhs[0..dot_index];
                let prop= &rhs[dot_index+1..];

                if obj.len() > 0&& prop.len() > 0 {
                    // process a call 
                    let obj_name = &obj[0];

                    if let Tokens::Token(t) = obj_name {
                        let res = Reserved::from_tendril(t);

                        match res {
                            Reserved::This => {
                                // context call 
                                // check if it is a method call on prop 
                                let call_index = prop.iter().position(|t| t == &Tokens::OpeningPar);

                                if let Some(_) = call_index {
                                    // it'sa function call 
                                    let name = &prop[0];

                                    if let Tokens::Token(tok) = name {
                                        block.set_value(Values::Call(FunctionCall::ContextCall(tok.clone())));
                                    }
                                }else{
                                    // it's a context property 
                                    let property = &prop[0];

                                    if let Tokens::Token(c) = property {
                                        block.set_value(Values::Property(ObjProp::Context(c.clone())));
                                    }
                                }
                            },
                            Reserved::Label(c) => {
                                // method call
                                // find the property call or name 
                                let call_index = prop.iter().position(|t| t == &Tokens::OpeningPar);

                                if let Some(_) = call_index {
                                    // a method callor property on function name 
                                    let name = &prop[0];

                                    if let Tokens::Token(tok) = name {
                                        block.set_value(Values::Call(FunctionCall::MethodCall(c.clone(), tok.clone())));
                                    }
                                }else{
                                    //Property 
                                    let property = &prop[0];

                                    if let Tokens::Token(t) = property {
                                        block.set_value(Values::Property(ObjProp::Object(c.clone(), t.clone())));
                                    }
                                }
                            },
                            _ => ()
                        }
                    }

                }
            }else{
                // a value
                // check for a list of object [..] 
                let square = rhs.iter().position(|t| t == &Tokens::OpeningSquare);

                if let Some(sq_index) = square {
                    if sq_index == 0 {
                        // list declaration 
                        // find the closing token 
                        let closing = rhs.iter().position(|t| t == &Tokens::ClosingSquare);

                        if let Some(closing) = closing {
                            // items between sq_index .. closing are vector items 
                            let items = &rhs[sq_index..closing];

                            let mut vec_items: Vec<Values> = vec![];

                            let mut str_count = 0;
                            let mut value = StrTendril::new();

                            for token in items.iter(){
                                // process the items for values 
                                match token {
                                    Tokens::StringDouble | Tokens::StringSingle => {
                                            // it is a string 
                                        if str_count == 2 {
                                            // record the value 
                                            vec_items.push(Values::Str(value.clone()));
                                            str_count = 0;
                                        }else{
                                            str_count += 1;
                                        }
                                    },
                                    Tokens::Token(c) => {
                                        // check if it undefined 
                                        if c == &StrTendril::from_slice("None"){
                                            vec_items.push(Values::Undefined);
                                        }else{
                                            // check for integers 
                                            if let Ok(num) = c.to_string().parse::<u32>(){
                                                vec_items.push(Values::Integer(num));
                                            }else{
                                                if let Ok(num) = c.to_string().parse::<f32>(){
                                                    vec_items.push(Values::Float(num))
                                                }else{
                                                    value = c.clone()
                                                }
                                            }
                                        }
                                    }
                                    _ => ()
                                }
                                
                            }

                            block.set_value(Values::List(vec_items));
                        }
                    }else{
                        // property access on an object
                        let closing = rhs.iter().position(|t| t == &Tokens::ClosingSquare);

                        if let Some(closing) = closing {
                            // get the object name and the property 
                            let obj_name = &rhs[0..sq_index][0];
                            let prop_name = &rhs[sq_index+1..closing][0];

                            if let Tokens::Token(c) = obj_name  {
                                if let Tokens::Token(n) = prop_name {
                                    block.set_value(Values::Property(ObjProp::Object(c.clone(), n.clone())))
                                }
                            }
                        }else{
                            // closing bracket not available 
                        }
                    }
                }else{
                    // check if it is a dict assignment 
                    println!("Tokens {:?}", rhs);
                    let tok = &rhs[0];

                    match tok {
                        Tokens::Token(c) => {
                            let res = Reserved::from_tendril(c);

                            match res {
                                Reserved::Label(c) => {
                                    // check the type
                                    if c == StrTendril::from_slice("None"){
                                        block.set_value(Values::Undefined);
                                    } else if let Ok(n) = c.to_string().parse::<u32>(){
                                        block.set_value(Values::Integer(n))
                                    }else if let Ok(f) = c.to_string().parse::<f32>() {
                                        block.set_value(Values::Float(f));
                                    }else{
                                        block.set_value(Values::Str(c));
                                    }
                                },
                                _ => ()
                            }
                        },
                        _ => ()
                    }
                }
                
            }


        }

        // insert the block 
        if let Some(builder) = self.builders.top_mut(){
            builder.add_block(Box::new(block));
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