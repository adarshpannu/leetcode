struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn buddy_strings(a: String, b: String) -> bool {
        if a.len() != b.len() {
            return false;
        }

        let diff: Vec<i128> = a
            .chars()
            .zip(b.chars())
            .map(|(ca, cb)| ca as i128 - cb as i128)
            .filter(|diff| *diff != 0)
            .collect();

        if diff.len() == 0 {
            let mut a_dedup: HashSet<char> = a.chars().collect();
            return a.len() != a_dedup.len();
        } else {
            return (diff.len() == 2) && (diff[0] + diff[1] == 0)
        }
    }
}

#[test]
fn test1() {
    use Solution;
    assert_eq!(Solution::buddy_strings("abcd".to_string(), "acbd".to_string()), true);
    assert_eq!(Solution::buddy_strings("aa".to_string(), "aa".to_string()), true);
    assert_eq!(Solution::buddy_strings("abc".to_string(), "abc".to_string()), false);
    assert_eq!(Solution::buddy_strings("abab".to_string(), "abab".to_string()), true);
}
