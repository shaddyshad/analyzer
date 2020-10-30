extern crate parser;

use parser::from_file;

fn main() {
    let fp = "/home/ank3r/Documents/ml/p37/parser/src/test.py";
    from_file(fp);
}
