use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;
use tendril::StrTendril;

pub mod tokenizer;

pub use tokenizer::{Tokenizer, TokenSink};


// Read a file by lines
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>
{
    let file = File::open(filename)?;

    Ok(io::BufReader::new(file).lines())
}

// open from a filename 
pub fn from_file(fp: &str) -> Result<Tokenizer, &'static str>{
    let mut tok = Tokenizer::new();

    if let Ok(lines) = read_lines(fp){
        // feed to tokenizer 
        for line in lines {
            if let Ok(l) = line {
                let mut buf = StrTendril::from(l);
                
                buf.push_char('\n');

                // feed to tokenizer  
                let _ = tok.feed(buf);
            }
        }

        Ok(tok)
    }else{
        Err("Error: File not found")
    }
}