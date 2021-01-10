use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }
        let diffs: Vec<(char, char)> = word1
            .chars()
            .zip(word2.chars())
            .filter(|(ch1, ch2)| ch1 != ch2)
            .collect();
        dbg!(diffs);
        false
    }
}

#[test]
fn test() {
    use Solution;

    Solution::close_strings("aacbb".to_string(), "ddcqb".to_string());
}
