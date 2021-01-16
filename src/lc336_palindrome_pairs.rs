struct Solution;

impl Solution {
    fn is_palindrome(word1: &Vec<char>, word2: &Vec<char>) -> bool {
        let len = word1.len() + word2.len();
        let mid = len / 2 as usize;

        let get_char = |ix: usize| {
            if ix < word1.len() {
                word1[ix]
            } else {
                word2[ix - word1.len()]
            }
        };
        (0..mid).all(|ix| get_char(ix) == get_char(len - 1 - ix))
    }

    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut retval: Vec<Vec<i32>> = Vec::new();
        let words: Vec<Vec<char>> = words.iter().map(|e| e.chars().collect()).collect();

        for (ix, word1) in words.iter().enumerate() {
            for (jx, word2) in words.iter().enumerate() {
                if ix != jx {
                    if Self::is_palindrome(word1, word2) {
                        let v = vec![ix as i32, jx as i32];
                        retval.push(v);
                    }
                }
            }
        }
        retval
    }
}

#[test]
fn test() {
    use Solution;

    let words = vec!["abcd", "dcba", "lls", "s", "sssll"];
    let words = words.iter().map(|e| e.to_string()).collect();
    dbg!(Solution::palindrome_pairs(words));
}
