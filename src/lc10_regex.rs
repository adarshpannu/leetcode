struct Solution;

/*
Given an input string (s) and a pattern (p), implement regular expression
matching with support for '.' and '*' where:

'.' Matches any single character.​​​​
'*' Matches zero or more of the preceding element.
The matching should cover the entire input string (not partial).

Input: s = "mississippi", p = "mis*is*p*."
Output: false

(START, m) -> 'i'

*/

enum RegexElem {
    Dot,
    Star(char),
    Specific(char)
}


impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        for ch in s.chars() {
        }
        false
    }
}

#[test]
fn test() {}
