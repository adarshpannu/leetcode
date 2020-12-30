struct Solution;

impl Solution {
    fn is_palindrome(v: &Vec<i32>) -> bool {
        (0..v.len()/2).into_iter().all(|ix| v[ix] == v[v.len() - 1 - ix])
    }

    pub fn nearest_palindromic(n: String) -> String {
        let mut nums: Vec<i32> = n.chars().map(|ch| ch as i32 - '0' as i32).collect();
        let len = nums.len();
        let mid = len / 2;

        dbg!(Solution::is_palindrome(&nums));

        if Solution::is_palindrome(&nums) {
            if nums[mid] > 0 {
                // 929
                nums[mid] -= 1;
            } else {
                // 0 | 5 | 909 | 101
                if mid == 0 {
                    // 0 | 5
                    if nums[mid] == 0 {
                        nums[mid] = 1
                    } else {
                        nums[mid] -= 1
                    }
                } else {
                    // 909 => 808
                    if nums[mid - 1] == 1 {
                        nums[mid - 1] -=  1;
                    } else {
                        nums[mid - 1] -=  1;
                    }
                }
            }
        }

        (0..nums.len()/2).into_iter().for_each(|ix| nums[len - 1 - ix] = nums[ix]);

        let mut retstr = String::new();
        for n in nums {
            retstr += &n.to_string()
        }
        dbg!(&retstr);
        retstr
    }
}

#[test]
fn test() {
    use Solution;

    assert_eq!(Solution::nearest_palindromic("1".to_string()), "0");
    assert_eq!(Solution::nearest_palindromic("0".to_string()), "1");
    assert_eq!(Solution::nearest_palindromic("121".to_string()), "111");
    assert_eq!(Solution::nearest_palindromic("101".to_string()), "111");


}
