struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        // remap to english alphabet
        let remapped_alphabet: HashMap<char, usize> = order
            .chars()
            .enumerate()
            .map(|(ix, ch)| (ch, ix))
            .collect();

        'toploop: for word_pair in words.windows(2) {
            for (ch1, ch2) in word_pair[0].chars().zip(word_pair[1].chars()) {
                match remapped_alphabet[&ch1].cmp(&remapped_alphabet[&ch2]) {
                    std::cmp::Ordering::Equal => {},
                    std::cmp::Ordering::Greater => return false,
                    std::cmp::Ordering::Less => continue 'toploop,
                }
            }
            if word_pair[0].len() > word_pair[1].len() {
                return false
            }
        }
        return true;
    }
}

#[test]
fn test() {
    use Solution;

    assert_eq!(
        Solution::is_alien_sorted(
            vec!["hellow".to_string(), "hello".to_string()],
            "hlabcdefgijkmnopqrstuvwxyz".to_string(),
        ),
        true
    );
}
