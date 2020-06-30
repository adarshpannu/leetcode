impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut rs = 0;
        nums.iter().map(|elem| {
            rs += *elem;
            rs
        }).collect::<Vec<i32>>()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::running_sum(vec!(1,2,3,4)), vec!(1,3,6,10));
    assert_eq!(Solution::running_sum(vec!()), vec!());
    assert_eq!(Solution::running_sum(vec!(1)), vec!(1));
    assert_eq!(Solution::running_sum(vec!(1,1,1,1,1)), vec!(1,2,3,4,5));

}

struct Solution {}
