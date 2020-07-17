impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut retval = 0;
        for (ix, &ex) in nums.iter().enumerate() {
            for (jx, &ej) in nums[ix+1..].iter().enumerate() {
                if (ex == ej) {
                    retval += 1;
                }
            }
        }
        retval
    }
}

struct Solution;

#[test]
fn test() {
    assert_eq!(Solution::num_identical_pairs(vec!(1,2,3,1,1,3)), 4);
    assert_eq!(Solution::num_identical_pairs(vec!(1,1,1,1)), 6);

}