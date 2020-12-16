// 1684. Count the Number of Consistent Strings

struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn count_consistent_strings0(allowed: &str, words: Vec<&str>) -> i32 {
        let words: Vec<String> = words.iter().map(|&str| str.to_string()).collect();
        Solution::count_consistent_strings(allowed.to_string(), words)
    }

    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let hs: HashSet<_> = allowed.chars().into_iter().collect();

        words
            .iter()
            .filter(|&str| {
                str.chars().all(|ch| hs.contains(&ch))
            })
            .count() as i32
    }
}

#[test]
fn test_1684() {
    use Solution;

    assert_eq!(
        Solution::count_consistent_strings0("ab", vec!("a", "b", "c", "ab", "ac", "bc", "abc")),
        3
    );

    assert_eq!(
        Solution::count_consistent_strings0("a", vec!("a", "b", "c", "ab", "ac", "bc", "abc")),
        1
    );
}
