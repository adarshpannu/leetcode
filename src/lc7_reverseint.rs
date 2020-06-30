// Ref: https://leetcode.com/problems/reverse-integer/

impl Solution {
    // 123 -> 321, -89 -> -98
    pub fn reverse(x: i32) -> i32 {
        let sign = if (x < 0) { -1 } else { 1 };
        let mut x = x.abs();
        let mut retval = 0i32;

        loop {
            let (q, r) = (x / 10, x % 10);
            let mut retval_opt = retval.checked_mul(10);
            if retval_opt.is_none() {
                return 0
            }
            retval = retval_opt.unwrap();
            retval_opt = retval.checked_add(r);
            if retval_opt.is_none() {
                return 0
            }
            retval = retval_opt.unwrap();

            x = q;
            if x == 0 {
                break;
            }
        }
        return sign * retval;
    }
}

struct Solution;

#[test]
fn test() {
    assert_eq!(Solution::reverse(123), 321);
    assert_eq!(Solution::reverse(-46), -64);
    assert_eq!(Solution::reverse(0), 0);
    assert_eq!(Solution::reverse(1), 1);
    assert_eq!(Solution::reverse(-1), -1);
    assert_eq!(Solution::reverse(1534236469), 0);
}
