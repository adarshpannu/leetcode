impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let n = n as usize;
        nums[0..n].iter().enumerate().flat_map(|(ix, elem)| vec![nums[ix], nums[ix + n]]).collect()
    }
}

struct Solution;

#[test]
fn test() {
    let v = vec![2, 5, 1, 3, 4, 7];
    assert_eq!(Solution::shuffle(v, 3), vec!(2, 3, 5, 4, 1, 7));

    let v = vec![];
    assert_eq!(Solution::shuffle(v, 0), vec!());
}
