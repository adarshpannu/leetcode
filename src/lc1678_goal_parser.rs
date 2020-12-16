/*
You own a Goal Parser that can interpret a string command. 
The command consists of an alphabet of "G", "()" and/or "(al)" in some order. 
The Goal Parser will interpret "G" as the string "G", "()" as the string "o", and "(al)" as the string "al". The interpreted strings are then concatenated in the original order.

*/

struct Solution;

impl Solution {
    fn interpret(str: String) -> String {
        let mut prev_ch = ' ';
        let mut ret_str = String::new();
        for ch in str.chars() {
            match (prev_ch, ch) {
                (_, 'G') => ret_str += "G",
                ('(', ')') => ret_str += "o",
                ('l', ')') => ret_str += "al",
                _ => {}
            }
            prev_ch = ch;
        }
        ret_str
    }
}

#[test]
fn test() {
    use Solution;

    assert_eq!(Solution::interpret("G()()(al)".to_string()), "Gooal".to_string());
}
