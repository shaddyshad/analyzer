use tendril::StrTendril;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tokens {
    Token(StrTendril),
    Tab,                // \t
    NewLine,            //\n
    OpeningPar,         // (
    ClosingPar,         //)
    StringSingle,       // '
    StringDouble,       // "
    Colon,              // :
    OpeningBrace,       // {
    ClosingBrace,       // }
    Comma,              // ,
    Dot,                // . 
    AngleRight,         // > 
    AngleLeft,          // < 
    Comment,            // # 
    Equals,             // =
    Star,               // *
    OpeningSquare,      // [
    ClosingSquare,      // ]
    Space,              // ' '
}