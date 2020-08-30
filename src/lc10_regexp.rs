/*
Given an input string (s) and a pattern (p), implement regular expression 
matching with support for '.' and '*'.

'.' Matches any single character.
'*' Matches zero or more of the preceding element.
The matching should cover the entire input string (not partial).

s could be empty and contains only lowercase letters a-z.
p could be empty and contains only lowercase letters a-z, and characters like . or *.


c*a*b* matches aab
*/

/* State Transitions: (State,Event) -> State 

Start + 'c' -> 
Start + 'a' -> 

*/

enum Event {
    Dot,
    Star(char),
    Char(char),
}


impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        unimplemented!()
    }

    pub fn compile(re: String) {
        for e in re.chars() {

        }
    }
}

struct Solution;

#[test]
fn test() {

}
