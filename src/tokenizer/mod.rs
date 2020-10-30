pub mod states;
#[macro_use]
pub mod small_charset;
pub mod input_buffer;
mod definitions;
pub mod sink;
pub mod tokens;
pub mod token_set;

pub use token_set::TokenSet;

pub use tokens::Tokens;
pub use sink::TokenSink;
use states::States;
pub use small_charset::SmallCharSet;
pub use input_buffer::{InputBuffer, SetResult};

use SetResult::{FromSet, NotFromSet};

use tendril::StrTendril;

pub struct Tokenizer {
    state: States,
    current_char: char,
    reconsume: bool,
    line_number: u32,
    buffer: InputBuffer,
    sink: TokenSink
}

impl Tokenizer {
    pub fn new() -> Self {
        Self {
            state: States::default(),
            current_char: '\0',
            reconsume: false,
            line_number: 0,
            buffer: InputBuffer::new(),
            sink: TokenSink::new()
        }
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
        while self.step(){
            println!("Round");
        }
    }

    fn emit(&mut self, token: Tokens){
        self.sink.process(token);
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
        println!("Processing in state {:?} ", self.state);

        match self.state {
            States::Document => loop{
                let set = TokenSet::new(vec![' ', '\n', '\t', '\'', '"', '{', '}', '(', ')', ':', '.', ',', '*', '#', '=']);

                match pop_from_set!(self, set){
                    FromSet(' ') => (),
                    FromSet('\n') => self.emit(Tokens::NewLine),
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
                    FromSet(_) => (),
                    NotFromSet(c) => self.emit(Tokens::Token(c))
 
                }
            },
           
            _ => {
                println!("Some shit!");

                false
            }
        }
    }
}