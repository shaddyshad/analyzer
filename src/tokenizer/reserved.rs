use tendril::StrTendril;
use regex::Regex;

#[derive(Debug)]
pub enum Reserved {
    Class,
    Def,
    If,
    For,
    This,
    Import,
    FromImport,
    Label(StrTendril)
}

//check if is an answer
pub fn matches<'a>(name: &'a str, pattern: &'static str) -> bool {
    let re = Regex::new(pattern).unwrap();

    return re.is_match(name);
}

impl Reserved {
    pub fn from_tendril(t: &StrTendril) -> Self {
        if matches(t, "class"){
            return Self::Class;
        }

        // defs 
        if matches(t, "def"){
            return Self::Def;
        }

        // if  
        if matches(t, "if"){
            return Self::If;
        }

        // for 
        if matches(t, "for"){
            return Self::For;
        }

        // this 
        if matches(t, "self"){
            return Self::This;
        }

        // import 
        if matches(t, "import"){
            return Self::Import;
        }

        // from .. import 
        if matches(t, "from"){
            return Self::FromImport;
        }

        Self::Label(t.clone())
    }
}