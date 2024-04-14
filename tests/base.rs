#[cfg(test)]
mod tests {
    use rusty_tokenizer::base;
    use std::char;
    use std::collections::HashMap;
    use std::fs::{self, File};
    use std::io::{BufRead, BufReader, Error, ErrorKind, Read, Write};

    fn load(
        model_file: String,
    ) -> Result<(String, HashMap<String, i32>, HashMap<(i32, i32), i32>), Error> {
        assert!(model_file.ends_with(".model"));

        let mut merges = HashMap::new();
        let mut special_tokens = HashMap::new();
        let mut idx = 256;

        let file = File::open(model_file)?;
        let reader = BufReader::new(file);

        let mut lines = reader.lines();

        let version = lines.next().unwrap()?.trim().to_string();
        assert_eq!(version, "minbpe v1");

        let pattern = lines.next().unwrap()?.trim().to_string();

        let num_special = lines.next().unwrap()?.trim().parse::<i32>().unwrap();
        for _ in 0..num_special {
            let line = lines.next().unwrap()?;
            let mut parts = line.trim().split_whitespace();
            let special = parts.next().unwrap().to_string();
            let special_idx = parts.next().unwrap().parse::<char>().unwrap();
            special_tokens.insert(special, special_idx as i32);
        }

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

    fn save(
        file_prefix: String,
        pattern: String,
        special_tokens: HashMap<i32, char>,
        merges: &HashMap<(i32, i32), i32>,
    ) -> Result<(), Error> {
        let model_file = file_prefix + ".model";

        let mut file = File::create(&model_file)?;

        writeln!(file, "minbpe v1")?;
        writeln!(file, "{}", &pattern)?;

        writeln!(file, "{}", special_tokens.len())?;
        for (special, idx) in special_tokens.iter() {
            writeln!(file, "{} {}", special, idx)?;
        }

        for ((idx1, idx2), _) in merges.iter() {
            writeln!(file, "{} {}", idx1, idx2)?;
        }

        Ok(())
    }

    // Test function for save
    #[test]
    fn test_save_and_load() {
        let file_prefix = String::from("test");
        let pattern = String::from("test pattern");
        let mut special_tokens = HashMap::new();
        special_tokens.insert(1, 'a');
        special_tokens.insert(2, 'b');
        let mut merges = HashMap::new();
        merges.insert((1, 2), 3);

        // Call the save function and assert the result
        let result = save(
            file_prefix.clone(),
            pattern.clone(),
            special_tokens.clone(),
            &merges,
        );
        assert!(result.is_ok());
        let model_file = file_prefix + ".model";
        assert!(std::path::Path::new(&model_file).exists());

        // testing load
        let model_file = String::from("test.model");
        match load(model_file.clone()) {
            Ok((pattern, special_tokens, merges)) => {
                println!("Pattern: {}", pattern);
                println!("special tokens: {:?}", special_tokens);
                println!("merges: {:?}", merges);
            }
            Err(e) => {
                eprintln!("Error loading model: {}", e);
            }
        }

        fs::remove_file(&model_file).expect("Failed to delete file");
    }

    #[test]
    fn test_get_stats_empty() {
        let ids: Vec<i32> = vec![];
        let counts = None;
        let result = base::get_stats(&ids, counts);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_get_stats() {
        let ids = vec![1, 2, 3, 1, 2];
        let mut m = HashMap::new();
        m = base::get_stats(&ids, Some(m));

        assert_eq!(m.len(), 3);
        let res = match m.get(&(1, 2)) {
            Some(num) => *num,
            None => 0,
        };
        assert_eq!(res, 2);
    }

    #[test]
    fn test_merge() {
        let ids = vec![1, 2, 3, 1, 2];
        let pair = (1, 2);
        let idx = 4;

        let newids = base::merge(&ids, pair, idx);
        assert_eq!(newids.len(), 3);
        assert_eq!(newids[0], 4);
        assert_eq!(newids[1], 3);
    }

    #[test]
    fn test_merge_nothing_to_replace() {
        let ids = vec![1, 2, 3, 4];
        let pair = (5, 6);
        let idx = 9;

        let newids = base::merge(&ids, pair, idx);
        assert_eq!(newids.len(), 4);
    }

    #[test]
    fn test_replace_control_characters() {
        let string_w_c_chars = "abc\nde\n";
        let replaced = base::replace_control_characters(&string_w_c_chars);
        assert_eq!(replaced, "abcde");
    }

    #[test]
    fn test_replace_control_characters_nothing_to_replace() {
        let string_no_chars = "abc";
        let replaced = base::replace_control_characters(&string_no_chars);
        assert_eq!(replaced, "abc");
    }
}
