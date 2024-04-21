//#![allow(warnings)]

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut ix = (nums1.len() - 1) as usize;
        let mut m = m as usize;
        let mut n = n as usize;
        while m > 0 || n > 0 {
            let ix = m + n - 1;
            if m > 0 && n > 0 {
                if nums1[m - 1] >= nums2[n - 1] {
                    nums1[ix] = nums1[m - 1];
                    m -= 1;
                } else {
                    nums1[ix] = nums2[n - 1];
                    n -= 1;
                }
            } else if m == 0 {
                nums1[ix] = nums2[n - 1];
                n -= 1;
            } else {
                nums1[ix] = nums1[m - 1];
                m -= 1;
            }
        }
    }
}

#[test]
pub fn test() {
    use Solution;
    let mut v1 = vec![1, 0];
    let mut v2 = vec![2];

    Solution::merge(&mut v1, 1, &mut v2, 1);
    dbg!(&v1);
}
struct Solution;
