struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut hmap: HashMap<char, usize> = HashMap::new();
        for (ix, ch) in s.chars().enumerate() {
            hmap.insert(ch, ix);
        }

        let mut split_ix = 0usize;
        let mut retvec = vec!();
        let mut count = 0;
        for (ix, ch) in s.chars().enumerate() {
            count += 1;
            split_ix = *hmap.get(&ch).unwrap().max(&split_ix);
            if ix == split_ix {
                retvec.push(count);
                count = 0;
            }
        }
        retvec
    }
}

#[test]
fn test() {
    use Solution;

    dbg!(Solution::partition_labels("ababcbacadefegdehijhklij".to_string()));
    dbg!(Solution::partition_labels("a".to_string()));

}