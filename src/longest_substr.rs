#![allow(warnings)]

// Ref: https://leetcode.com/problems/longest-substring-without-repeating-characters/

/*
Given a string, find the length of the longest substring without repeating characters.
*/

fn is_end_of_substring(s: &str, ix: usize) -> bool {
    if ix >= s.len() - 1 {
        return true
    } else {
        (s[ix..ix+1] == s[(ix + 1)..(ix + 2)])
    }
}

#[test]
fn run() {
    let s = "helllabcdddeeerer";
    println!("is_repeated: {}", is_end_of_substring(s, 11));
    let mut ix = 0;
    let mut cur_interval = [0, 0];
    let mut max_interval = [0, 0];
    let mut in_interval = true;

    while ix < s.len() {
        if in_interval && is_end_of_substring(s, ix) {
            // end of interval
            cur_interval[1] = ix;
            in_interval = false;
            println!("{:?} {:?}", cur_interval, &s[cur_interval[0]..cur_interval[1]+1]);

        } else if ! in_interval && ! is_end_of_substring(s, ix) {
            // start of new interval
            cur_interval[0] = ix;
            in_interval = true;
        }
        ix += 1;
    }
}
