use std::{collections::HashMap, hash::Hash};

use crate::base::*;

struct BasicTokenizer {
    merges: HashMap<(i32, i32), i32>,
    pattern: String,
    special_tokens: HashMap<String, i32>,
}

impl Tokenizer for BasicTokenizer {
    fn train(&self, text: String, vocab_size: i32, verbose: bool) {
        assert!(
            vocab_size >= 256,
            "Vocab size must be greater than or equal to 256."
        );

        let num_merges = vocab_size - 256;
        let text_bytes = text.as_bytes();
        let ids: Vec<i32> = text_bytes.iter().map(|&b| b as i32).collect();

        let merges: HashMap<(i32, i32), i32> = HashMap::new();
        let mut vocab: HashMap<i32, Vec<u8>> = HashMap::new();
        for idx in 0..256 {
            vocab.insert(idx, vec![idx as u8]);
        }

        for _ in 0..num_merges {
            let stats = get_stats(&ids, None);
        }
    }
}
