struct Solution;

impl Solution {
    fn is_palindrome(str: String) -> bool {
        let chars: Vec<char> = str.chars().collect();
        chars
            .iter()
            .zip(chars.iter().rev())
            .all(|(ch1, ch2)| *ch1 == *ch2)
    }

    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut retval: Vec<Vec<i32>> = Vec::new();

        for (ix, word1) in words.iter().enumerate() {
            for (jx, word2) in words.iter().enumerate() {
                if ix != jx {
                    let word = format!("{}{}", word1, word2);
                    if Self::is_palindrome(word) {
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

    dbg!(Solution::is_palindrome("abba".to_owned()));
}
