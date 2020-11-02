pub mod states;
pub mod input_buffer;
pub mod sink;
pub mod tokens;
pub mod token_set;
pub mod reserved;
pub mod stack;
pub mod interface;
pub use interface::{Class, PyEntity};
pub use stack::Stack;
pub mod line;
pub use reserved::Reserved;
pub use line::{Line, LineTypes};
pub use token_set::TokenSet;

pub use tokens::Tokens;
pub use sink::TokenSink;
use states::States;
pub use input_buffer::{InputBuffer, SetResult};

use SetResult::{FromSet, NotFromSet};

use tendril::StrTendril;

pub struct Tokenizer{
    state: States,
    current_char: char,
    reconsume: bool,
    line_number: u32,
    buffer: InputBuffer,
    sink: TokenSink,
    tokens: Vec<Tokens>,
}

impl Tokenizer {
    pub fn new() -> Self {
        Self {
            state: States::default(),
            current_char: '\0',
            reconsume: false,
            line_number: 0,
            buffer: InputBuffer::new(),
            sink: TokenSink::new(),
            tokens: vec![]
        }
    }

    pub fn sink(&self) -> &TokenSink {
        &self.sink
    }

    pub fn get_char(&mut self) -> Option<char>{
        if self.reconsume{
            self.reconsume = false;
            Some(self.current_char)
        }else{
            self.buffer.next()
                .and_then(|c| self.get_processed_char(c))
        }
    }

    fn get_processed_char(&mut self, c: char) -> Option<char>{
        // handle new lines 
        if c == '\n'{
            self.line_number += 1;
        }

        self.current_char = c;

        Some(c)
    }

    fn pop_except_from(&mut self, set: TokenSet) -> Option<SetResult>{
        if self.reconsume{
            return self.get_char().map(|x| FromSet(x));
        }

        let d = self.buffer.pop_except_from(set);

        match d {
            Some(FromSet(c)) => self.get_processed_char(c).map(|c| FromSet(c)),
            _ => d
        }
    }

    pub fn feed(&mut self, buffer: StrTendril) -> bool {
        if buffer.is_empty(){
            return false;
        }

        self.buffer.push(buffer);

        self.run();

        true 
    }

    fn run(&mut self){
        while self.step(){}
    }

    // commit a line 
    fn commit_line(&mut self){
        // get all tokens 
        let line = Line {
            tokens: self.tokens.clone(),
            line_number: self.line_number
        };

        // commit this line 
        self.sink.process(line);

        // clear the vector 
        self.tokens.clear();
    }

    // emit a token 
    fn emit(&mut self, token: Tokens){
        // insert a token 
        self.tokens.push(token);
    }

}


macro_rules! unwrap_or_else(
    ($opt:expr, $else_block:block) => {
        match $opt {
            None => $else_block,
            Some(c) => c,
        }
    }
);

macro_rules! unwrap_or_return(
    ($opt:expr, $retval:expr) => {
        unwrap_or_else!($opt, {return $retval })
    }
);

/// Macros used by the tokenizer
macro_rules! pop_from_set(
    (  $me:expr, $set:expr) => (
        unwrap_or_return!($me.pop_except_from($set), false)
    )
);


impl Tokenizer {
    pub fn step(&mut self) -> bool {

        match self.state {
            States::Document => loop{
                let set = TokenSet::new(vec![' ', '\n', '\t', '\'', '"', '{', '}', '(', ')', ':', '.', ',', '*', '#', '=', '[', ']']);

                match pop_from_set!(self, set){
                    FromSet(' ') => self.emit(Tokens::Space),
                    FromSet('\n') => self.commit_line(),
                    FromSet('\t') => self.emit(Tokens::Tab),
                    FromSet('(') => self.emit(Tokens::OpeningPar),
                    FromSet(')') => self.emit(Tokens::ClosingPar),
                    FromSet(':') => self.emit(Tokens::Colon),
                    FromSet('{') => self.emit(Tokens::OpeningBrace),
                    FromSet('}') => self.emit(Tokens::ClosingBrace),
                    FromSet('.') => self.emit(Tokens::Dot),
                    FromSet(',') => self.emit(Tokens::Comma),
                    FromSet('#') => self.emit(Tokens::Comment),
                    FromSet('*') => self.emit(Tokens::Star),
                    FromSet('=') => self.emit(Tokens::Equals),
                    FromSet('\'') => self.emit(Tokens::StringSingle),
                    FromSet('"') => self.emit(Tokens::StringDouble),
                    FromSet('[') => self.emit(Tokens::OpeningSquare),
                    FromSet(']') => self.emit(Tokens::ClosingSquare),
                    FromSet(_) => (),
                    NotFromSet(c) => self.emit(Tokens::Token(c))
 
                }
            },
           
            _ => false
        }
    }
}