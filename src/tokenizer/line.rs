use super::{Tokens, Reserved};
use tendril::StrTendril;
// a single line emitted 
#[derive(Debug)]
pub struct Line {
    pub tokens: Vec<Tokens>,
    pub line_number: u32
}

// types of lines in a python script 
#[derive(Debug)]
pub enum LineTypes {
    ClassDeclaration,
    FunctionCreation,
    Assignment,
    Condition,
    Loop,
    MethodCall,
    FunctionCall,
    ObjectCreation,
    Text(StrTendril),
    DocString,
    Comment,
    Empty
}



impl Line {
    pub fn line_type(&self) -> LineTypes {
        // determine each line type
        let tokens = self.trim();

        // check for a doctring 
        if self.subsequent_quotes() == 3 {
            return LineTypes::DocString;
        }

        // check for an assignment by presence of '=' and not subsequent 
        if self.is_assignment(&tokens) {
            return LineTypes::Assignment;
        }

        if let Some(token) = tokens.first(){
            if let Tokens::Token(ref c) = token {
                // check class declaration 
                let declaration = Reserved::from_tendril(c);

                match declaration {
                    Reserved::Class => return LineTypes::ClassDeclaration,
                    Reserved::Def => return LineTypes::FunctionCreation,
                    Reserved::If => return LineTypes::Condition,
                    Reserved::For => return LineTypes::Loop,
                    _ => return LineTypes::Text(c.clone())
                }

            }

            // detect a comment  
            if let Tokens::Comment = token {
                return LineTypes::Comment;
            } 
        }
        
        LineTypes::Empty

    }

    // get the number of subsequent " or ' 
    fn subsequent_quotes(&self) -> u32 {
        let tokens = self.trim();
        let mut sub_double = 0;
        let mut sub_single = 0;

        for token in tokens.iter() {
            match token {
                Tokens::StringDouble => {
                    sub_double += 1;
                    if sub_double == 3 {
                        return 3;
                    }
                    sub_single = 0;
                },
                Tokens::StringSingle => {
                    sub_single += 1;

                    if sub_single == 3{
                        return 3;
                    }

                    sub_double = 0;
                },
                _ => {
                    sub_single = 0;
                    sub_double = 0;
                }
            }
        }

        sub_double | sub_single
    }

    pub fn depth(&self) -> u32 {
        // find the depth of this line 
        let mut depth = 0;
        let mut spaces = 0;

        for token in self.tokens.iter(){
            // check for spaces 
            if let Tokens::Space = token {
                spaces += 1;

                if spaces == 4 {
                    // increase the depth 
                    depth += 1;
                    spaces = 0;
                }
            }else{
                spaces = 0;
            }
        }

        depth 
    }


    // trim the space tokens 
    pub fn trim(&self) -> Vec<Tokens> {
        let mut tokens: Vec<Tokens> = vec![];

        for token in self.tokens.iter(){
            if let Tokens::Space = token {}
            else{
                tokens.push(token.clone())
            }
        }

        tokens 
    }

    // check for assignments 
    fn is_assignment(&self, tokens: &Vec<Tokens>) -> bool {
        let mut s = 0;

        for token in tokens.iter(){
            if let Tokens::Equals = token {
                s += 1;
            }
        }

        s == 1 
    }
}
