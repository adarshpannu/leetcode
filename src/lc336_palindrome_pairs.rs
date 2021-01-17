#![allow(warnings)]

use std::cmp::Ordering;
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
        //let words: Vec<Vec<char>> = words.iter().map(|e| e.chars().collect()).collect();

        let mut words = words.iter().enumerate().flat_map(|(ix, w)| {
            let mut wrev = w.clone();
            let wrev: String = w.chars().rev().collect();
            vec![(w.clone(), ix, "orig"), (wrev, ix, "rev")]
        }).collect::<Vec<(String,usize,&str)>>();

        words.sort_by(|a, b| {
            let cmp = a.0.cmp(&b.0);
            if cmp == Ordering::Equal {
                a.2.cmp(&b.2)
            } else {
                cmp
            }
        });

        dbg!(words);
        retval
    }
}

#[test]
fn test() {
    use Solution;

    let words = vec!["abx", "ba", "lls", "s", "sssll", "dxba"];
    //let words = vec!["a", ""];

    let words = words.iter().map(|e| e.to_string()).collect();
    dbg!(Solution::palindrome_pairs(words));

    let s = String::from("abc");
    let ss = &s[0..2];
}
