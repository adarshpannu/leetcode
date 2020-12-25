struct Solution;

impl Solution {
    // Negate largest -ve number then smallest positive number
    // 5, 1, 8, -5, -2, 0 => -5, -2, 0, 4, 5, 8
    pub fn largest_sum_after_k_negations(a: Vec<i32>, k: i32) -> i32 {
        let mut a = a.clone();
        let mut sum = 0;
        let mut k = k;
        let mut flip_elem = 0i32;

        a.sort_by(|a, b| a.cmp(b));
        for (ix, &elem) in a.iter().enumerate() {
            let mut sign = 1;
            if elem < 0 {
                if k > 0 {
                    sign = -1;
                    k -= 1
                }
            } else {
                k = k % 2;
            }
            let elemx = sign * elem;
            sum += elemx;
            if ix == 0 || elem.abs() < flip_elem.abs() {
                flip_elem = elemx;
            }
        }
        k = k % 2;
        if k == 1 {
            sum = sum - (2 * flip_elem)
        }
        sum
    }
}

#[test]
fn test() {
    assert_eq!(Solution::largest_sum_after_k_negations(vec!(-3, -2, 0, 3), 2), 8);
    assert_eq!(Solution::largest_sum_after_k_negations(vec!(-3, -2, 0, 5, 8), 4), 18);
    assert_eq!(Solution::largest_sum_after_k_negations(vec!(-3, -2, 1, 5, 8), 5), 17);
    assert_eq!(Solution::largest_sum_after_k_negations(vec!(-3, -2, 4, 5, 8), 5), 18);


}

