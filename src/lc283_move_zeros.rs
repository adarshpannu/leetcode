struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        // 10, 0, 5, 0, 2, 3
        let mut ix = 0;
        let mut jx = 1;

        while (ix < nums.len()) && (jx < nums.len()) {
            match (nums[ix], nums[jx]) {
                (0, 0) => {}
                (0, _) => {
                    nums.swap(ix, jx);
                    ix += 1;
                }
                (_, _) => {
                    ix += 1;
                }
            }
            jx += 1;
        }
    }
}

#[test]
fn unit_tests() {
    use Solution;

    let mut v = vec![1, 0, 3, 0, 4, 5, 0, 0];
    //let mut v = vec![0, 1, 0, 3, 12];
    println!("{:?}", v);
    Solution::move_zeroes(&mut v);
    println!("{:?}", v);
}
