extern crate parser;

use parser::from_file;

fn main() {
    let fp = "/home/ank3r/Documents/ml/p37/parser/src/test.py";
    if let Ok(tok) = from_file(fp){
        let sink = tok.sink();

        println!("Sink shape {:#?}", sink);
    }
}
