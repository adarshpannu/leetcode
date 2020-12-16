struct Solution;

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        for ix in 0..nums.len() {
            let jxmax = (ix + 1 + k as usize).min(nums.len());
            for jx in (ix + 1)..(jxmax) {
                println!("ix = {}, jx = {}", ix, jx);
                let diff = (nums[ix] as i128 - nums[jx] as i128).abs() as i128;
                if diff <= t as i128 {
                    return true;
                }
            }
        }
        false
    }

    pub fn contains_nearby_almost_duplicate2(nums: Vec<i32>, k: i32, t: i32) -> bool {

        println!("{:?} <- unsorted", nums);

        let mut diffs: Vec<(usize, &i32)> = nums.iter().enumerate().collect();

        diffs.sort_by(|&a, &b| a.1.cmp(b.1));

        println!("{:?} <- sorted", diffs);

        let mut prev_elem = *diffs[0].1;
        let mut cum_diff = 0;

        let diffs: Vec<(usize, i32)> = diffs
            .iter()
            .map(|&e| {
                let elem = *e.1 - prev_elem;
                cum_diff += elem;
                prev_elem = *e.1;
                (e.0, cum_diff)
            })
            .collect();
        println!("{:?} <- cum diffs", diffs);

        true
    }
}

 #[test]
fn test() {
    use Solution;

    //    14,       9,       2,       15,      29,       7 <- unsorted
    //   (2,2),   (5,7),   (1,9),   (0,14),  (3,15),  (4,29) <- sorted (ix, value)
    //   (2,0),   (5,5),   (1,2),    (0,5),  (3,1),   (4,14) <- abs diffs (ix, value)
    //   (2,0),   (5,5),   (1,7),    (0,12), (3,13),  (4,27) <- cum diffs (ix, value)

    assert_eq!(
        Solution::contains_nearby_almost_duplicate2(vec![14, 9, 2, 15, 29, 31], 1, 10),
        true
    );

    /*
    assert_eq!(Solution::contains_nearby_almost_duplicate(vec![1,2,3,1], 3, 0), true);
    assert_eq!(Solution::contains_nearby_almost_duplicate(vec![1, 5, 9, 3], 1, 2), true);
    assert_eq!(
        Solution::contains_nearby_almost_duplicate(vec![1, 5, 9, 1, 5, 9], 2, 3),
        false
    );
    */
}
