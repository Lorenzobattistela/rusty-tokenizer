#![allow(dead_code)]

use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read, Write};
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
pub trait Tokenizer {
    fn train(&mut self, text: String, vocab_size: i32, verbose: bool) {
        unimplemented!();
    }

    fn encode(&self, text: String) -> Vec<i32> {
        unimplemented!();
    }

    fn decode(&self, ids: &[i32]) -> String {
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
        &self,
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

    fn load(
        &self,
        model_file: String,
    ) -> Result<(String, HashMap<String, i32>, HashMap<(i32, i32), i32>), Error> {
        assert!(model_file.ends_with(".model"));

        let mut merges = HashMap::new();
        let mut special_tokens = HashMap::new();
        let mut idx = 256;

        let file = File::open(model_file)?;
        let reader = BufReader::new(file);

        let mut lines = reader.lines();

        // Read the version
        let version = lines.next().unwrap()?.trim().to_string();
        assert_eq!(version, "minbpe v1");

        // Read the pattern
        let pattern = lines.next().unwrap()?.trim().to_string();

        // Read the special tokens
        let num_special = lines.next().unwrap()?.trim().parse::<i32>().unwrap();
        for _ in 0..num_special {
            let line = lines.next().unwrap()?;
            let mut parts = line.trim().split_whitespace();
            let special = parts.next().unwrap().to_string();
            let special_idx = parts.next().unwrap().parse::<char>().unwrap();
            special_tokens.insert(special, special_idx as i32);
        }

        // Read the merges
        for line in lines {
            let line = line?;
            let mut parts = line.trim().split_whitespace();
            let idx1 = parts.next().unwrap().parse::<i32>().unwrap();
            let idx2 = parts.next().unwrap().parse::<i32>().unwrap();
            merges.insert((idx1, idx2), idx);
            idx += 1;
        }

        Ok((pattern, special_tokens, merges))
    }
}
