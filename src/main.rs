use std::collections::HashMap;

mod base;

fn main() {
    let c = 'h'.escape_unicode();
    println!("{}", c);
    println!("{}", 'h' as u32);

    let mut m = HashMap::new();

    let mut i = vec![1, 2, 3, 1, 2];

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
}
