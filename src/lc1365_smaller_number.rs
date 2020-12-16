struct Solution;

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .map(|&elem_outer| {
                nums.iter()
                    .filter(|&elem_inner| *elem_inner < elem_outer)
                    .count() as i32
            })
            .collect()
    }
}

#[test]
fn test() {}
