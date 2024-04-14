#![allow(dead_code)]

use std::fs::File;
use std::io::{Error, Write};
use std::{collections::HashMap, str::from_utf8};
use unicode_segmentation::UnicodeSegmentation;

pub fn get_stats(
    ids: &[i32],
    counts: Option<HashMap<(i32, i32), i32>>,
) -> HashMap<(i32, i32), i32> {
    // given a list of ints, return a hashmap of counts of consecutive pairs
    // optionally allows to update an existing dict of counts
    let mut counts = match counts {
        Some(map) => map,
        None => HashMap::new(),
    };

    for pair in ids.iter().zip(ids.iter().skip(1)) {
        *counts.entry((*pair.0, *pair.1)).or_insert(0) += 1;
    }
    counts
}

pub fn merge(ids: &[i32], pair: (i32, i32), idx: i32) -> Vec<i32> {
    // in the list of integers (ids), replace all consecutive occurrences of pair with the new
    // integer token idx. Example: ids = [1, 2, 3, 1, 2], pair=(1,2), idx=4 -> [4, 3, 4]

    let mut newids: Vec<i32> = vec![];
    let mut i = 0;

    while i < ids.len() {
        // if not at the very last position AND the pair matches, replace it
        if ids[i] == pair.0 && i < ids.len() - 1 && ids[i + 1] == pair.1 {
            newids.push(idx);
            i += 2;
        } else {
            newids.push(ids[i]);
            i += 1;
        }
    }
    newids
}

pub fn replace_control_characters(s: &str) -> String {
    s.graphemes(true)
        .filter(|&ch| !ch.chars().all(|c| c.is_control()))
        .collect::<String>()
}

pub fn render_token(t: &[u8]) -> String {
    // pretty print a token escaping control chars
    let s = match from_utf8(t) {
        Ok(s) => s,
        Err(_) => return String::from("Invalid UTF-8"),
    };
    replace_control_characters(s)
}

fn bytes(input: &str) -> Vec<u8> {
    input.bytes().collect()
}

fn bytes_char(input: char) -> Vec<u8> {
    vec![input as u8]
}

fn bytes_integers(input: u8) -> Vec<u8> {
    vec![input]
}

// base tokenizer

struct Tokenizerr {
    merges: HashMap<(i32, i32), i32>,
    pattern: String,
    special_tokens: HashMap<String, i32>,
    vocab: HashMap<i32, Vec<u8>>,
}

trait Tokenizer {
    fn train(&self, text: String, vocab_size: i32, verbose: bool) {
        unimplemented!();
    }

    fn encode(&self, text: String) {
        unimplemented!();
    }

    fn decode(&self, ids: Vec<i32>) {
        unimplemented!();
    }

    fn _build_vocab(&self, merges: &HashMap<(i32, i32), i32>) -> HashMap<i32, Vec<u8>> {
        let mut vocab: HashMap<i32, Vec<u8>> = HashMap::new();
        for idx in 0..256 {
            vocab.insert(idx, bytes_integers(idx as u8));
        }

        for ((p0, p1), &idx) in merges.iter() {
            let vocab_p0 = vocab.get(p0).map_or_else(|| vec![], |v| v.clone());
            let vocab_p1 = vocab.get(p1).map_or_else(|| vec![], |v| v.clone());
            let merged_bytes = [&vocab_p0[..], &vocab_p1[..]].concat();
            vocab.insert(idx, merged_bytes);
        }

        vocab
    }

    fn save(
        file_prefix: String,
        pattern: String,
        special_tokens: HashMap<i32, char>,
        merges: &HashMap<(i32, i32), i32>,
    ) -> Result<(), Error> {
        let model_file = file_prefix + ".model";

        let mut file = File::create(&model_file)?;

        writeln!(file, "minbpe v1\n")?;
        writeln!(file, "{}\n", &pattern)?;

        writeln!(file, "{}\n", special_tokens.len())?;
        for (special, idx) in special_tokens.iter() {
            writeln!(file, "{} {}", special, idx)?;
        }

        for ((idx1, idx2), _) in merges.iter() {
            writeln!(file, "{} {}", idx1, idx2)?;
        }

        Ok(())
    }
}
