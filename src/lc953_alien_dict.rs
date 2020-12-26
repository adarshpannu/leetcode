struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        // remap to english alphabet
        let remapped_alphabet: HashMap<char, char> = order
            .chars()
            .enumerate()
            .map(|(ix, ch)| (ch, ('a' as u8 + ix as u8) as char))
            .collect();

        let mut prev_remapped_word: String = "".to_string();
        for (ix, word) in words.iter().enumerate() {
            // remap word
            let remapped_word: String = word
                .chars()
                .map(|ch| remapped_alphabet[&ch])
                .collect();
            if ix > 0 && prev_remapped_word > remapped_word {
                return false;
            }
            prev_remapped_word = remapped_word;
        }
        return true;
    }
}

#[test]
fn test() {
    use Solution;

    assert_eq!(
        Solution::is_alien_sorted(
            vec!["hello".to_string(), "leetcode".to_string()],
            "hlabcdefgijkmnopqrstuvwxyz".to_string(),
        ),
        true
    );
}
