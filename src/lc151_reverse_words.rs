use itertools::Itertools;

// 151. Reverse Words in a String

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let words: Vec<_> = s.split(' ').filter(|e| e.len() > 0).rev().collect();
        words.join(" ")
    }
}

#[test]
fn test() {
    use Solution;
    dbg!(Solution::reverse_words("hello  world".to_string()));

}

struct Solution;
