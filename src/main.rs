use std::collections::HashMap;

use rusty_tokenizer::base::Tokenizer;
use rusty_tokenizer::basic::{self, BasicTokenizer};
use std::fs::File;
use std::io::Read;
use std::path::Path;

mod base;

fn main() {
    let c = 'h'.escape_unicode();
    println!("{}", c);
    println!("{}", 'h' as u32);

    let mut m = HashMap::new();

    let i = vec![1, 2, 3, 1, 2];

    m = base::get_stats(&i, Some(m));
    println!("{:?}", m);

    let ids = vec![1, 2, 3, 1, 2];
    let pair = (1, 2);
    let idx = 4;

    let newids = base::merge(&ids, pair, idx);
    println!("{:?}", newids);

    let input = "abc\n\x1b[31mdef\nghi\x1b[0m";
    let result = base::replace_control_characters(input);
    println!("{}", result); // Output: abc\ndef\nghi

    let token = b"abc\x1b[31mdef\x1b[0m";
    let result = base::render_token(token);
    println!("{}", result); // Output: abcdef

    let file_path = "tests/taylorswift.txt";
    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file: {}", err);
            return;
        }
    };

    let mut text = String::new();
    if let Err(err) = file.read_to_string(&mut text) {
        eprintln!("Error reading file: {}", err);
        return;
    }
    println!("{}", text);

    let mut tknz = BasicTokenizer::new();
    println!("{:?}", tknz);
    tknz.init_vocab();
    // println!("{:?}", tknz);
    tknz.train(text, 512, true);
    println!("{:?}", tknz);
}
