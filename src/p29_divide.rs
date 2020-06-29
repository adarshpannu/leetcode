// Ref: https://leetcode.com/problems/divide-two-integers/
const MIN: i32  = -2147483648;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if (dividend == MIN) || (divisor == MIN) {
            /*
            if (dividend == MIN) && (divisor == MIN) {
                return 1;
            } else if (dividend == MIN) {
                return 0;
            }
            */
            return 2147483647;
        }

        let sign = if (dividend < 0) ^ (divisor < 0) { -1 } else { 1 };
        let mut dividend = dividend.abs();
        let mut divisor = divisor.abs();

        let mut retval = 0;
        if dividend == 0 {
            return 0; // result undefined
        }
        while dividend >= divisor {
            retval += 1;
            dividend -= divisor;
        }
        sign * retval
    }
}

#[test]
fn test() {
    /*
    assert_eq!(10 / 2, Solution::divide(10, 2));
    assert_eq!(10 / 3, Solution::divide(10, 3));
    assert_eq!(1 / 3, Solution::divide(1, 3));
    assert_eq!(0 / 3, Solution::divide(0, 3));
    assert_eq!(-10 / 2, Solution::divide(-10, 2));
    assert_eq!(-10 / 3, Solution::divide(-10, 3));
    assert_eq!(-1 / 3, Solution::divide(-1, 3));

    assert_eq!(7 / -3, Solution::divide(7, -3));
    */

    //assert_eq!(-2147483647 / -1, Solution::divide(-2147483648, -1));
    println!("{}", -2147483648 / -1);
}

pub struct Solution;
