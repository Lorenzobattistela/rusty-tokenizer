use std::{collections::HashMap, hash::Hash};

use crate::base::*;

#[derive(Debug)]
pub struct BasicTokenizer {
    merges: HashMap<(i32, i32), i32>,
    pattern: String,
    special_tokens: HashMap<String, i32>,
    vocab: HashMap<i32, Vec<u8>>,
}

impl BasicTokenizer {
    pub fn new() -> Self {
        Self {
            merges: HashMap::new(),
            pattern: String::new(),
            special_tokens: HashMap::new(),
            vocab: HashMap::new(),
        }
    }

    pub fn init_vocab(&mut self) {
        self.vocab = self._build_vocab(&self.merges);
    }
}

impl Tokenizer for BasicTokenizer {
    fn train(&mut self, text: String, vocab_size: i32, verbose: bool) {
        assert!(
            vocab_size >= 256,
            "Vocab size must be greater than or equal to 256."
        );

        let num_merges = vocab_size - 256;
        let text_bytes = text.as_bytes();
        let mut ids: Vec<i32> = text_bytes.iter().map(|&b| b as i32).collect();

        let mut merges: HashMap<(i32, i32), i32> = HashMap::new();
        let mut vocab: HashMap<i32, Vec<u8>> = HashMap::new();
        for idx in 0..256 {
            vocab.insert(idx, vec![idx as u8]);
        }

        for i in 0..num_merges {
            let stats = get_stats(&ids, None);
            let pair = stats.iter().max_by_key(|(_, &v)| v).unwrap().0;

            let idx = 256 + i;
            ids = merge(&ids, *pair, idx);

            merges.insert(*pair, idx);

            let summed_bytes = vocab[&pair.0]
                .iter()
                .zip(vocab[&pair.1].iter())
                .map(|(&a, &b)| a.wrapping_add(b))
                .collect();

            vocab.insert(idx, summed_bytes);

            if verbose {
                println!(
                    "Merge {:?}/{:?}: {:?} -> {:?} ({:?} had {:?} occurrences)",
                    i + 1,
                    num_merges,
                    pair,
                    idx,
                    vocab[&idx],
                    stats[pair]
                );
            }
        }
        self.merges = merges;
        self.vocab = vocab;
    }

    fn decode(&self, ids: Vec<i32>) -> String {
        todo!()
    }

    fn encode(&self, text: String) -> Vec<i32> {
        todo!()
    }
}
