#[cfg(test)]
mod tests {
    use rusty_tokenizer::base;
    use std::collections::HashMap;

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
