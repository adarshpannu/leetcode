impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        let r = n.chars().into_iter().max().unwrap();
        (r as i32 - '0' as i32)        
    }
}

#[test]
pub fn test() {
    use Solution;
    let mut v1 = vec![1, 0];
    let mut v2 = vec![2];

    dbg!(Solution::min_partitions("421444".to_owned()));
}

struct Solution;
