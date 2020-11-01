use super::Tokens;
// a single line emitted 
#[derive(Debug)]
pub struct Line {
    pub tokens: Vec<Tokens>,
    pub line_number: u32
}

impl Line {
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
}
