use std::default::Default;

/// States enum
#[derive(Debug)]
pub enum States {
    Document,
    Token,
    NewLine,
    Tab,
    EoF
}

impl Default for States{
    fn default() -> Self {
        Self::Document
    }
}