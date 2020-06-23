#![allow(warnings)]

// Ref: https://leetcode.com/problems/longest-substring-without-repeating-characters/

/*
Given a string, find the length of the longest substring without repeating characters.
*/

fn is_next_char_repeated(s: &str, ix: usize) -> bool {
    // Name says it all. The only catch is that the last character returns 'true.' That's a bit odd.
    if ix >= s.len() - 1 {
        return true
    } else {
        (s[ix..ix+1] == s[(ix + 1)..(ix + 2)])
    }
}

#[test]
fn run() {
    let s = "helllabcdddeeererer";
    let mut ix = 0;
    let mut cur_interval = [0, 0];
    let mut max_interval = [0, 0];
    let mut in_interval = true;

    while ix < s.len() {
        if in_interval && is_next_char_repeated(s, ix) {
            // end of interval
            cur_interval[1] = ix + 1;
            in_interval = false;
            println!("{:?} {:?}", cur_interval, &s[cur_interval[0]..cur_interval[1]]);

            if (cur_interval[1] - cur_interval[0]) > (max_interval[1] - max_interval[0]) {
                max_interval = cur_interval;
            }
        } else if ! in_interval && ! is_next_char_repeated(s, ix) {
            // start of new interval
            cur_interval[0] = ix;
            in_interval = true;
        }
        ix += 1;
    }

    println!("Longest substring: {:?}", &s[max_interval[0]..max_interval[1]]);

}
#[test]
fn run2() {
    let s = "x";
    println!("Longest substring: {:?}", &s[0..0]);
}

