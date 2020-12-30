struct Solution;

impl Solution {
    fn to_digits(n: &String) -> Vec<i32> {
        n.chars()
            .map(|ch| ch as i32 - '0' as i32)
            .collect::<Vec<i32>>()
    }

    fn reflect(digits: &mut Vec<i32>) {
        let len = digits.len();
        let imid = len / 2;
        (0..imid).into_iter().for_each(|ix| {
            digits[len - 1 - ix] = digits[ix];
        });
    }

    fn is_palindrome(v: &Vec<i32>) -> bool {
        (0..v.len() / 2)
            .into_iter()
            .all(|ix| v[ix] == v[v.len() - 1 - ix])
    }

    fn to_num(v: &Vec<i32>) -> i32 {
        v.into_iter().fold(0, |acc, e| acc * 10 + e)
    }

    pub fn nearest_palindromic(n: String) -> String {
        dbg!(&n);

        let imid = n.len() / 2;
        let orignum = n.parse::<i32>().unwrap();
        let digits = Self::to_digits(&n);

        // Test cases: 0, 1, 101, 909, 797, 10001, 999
        // mid-1, mid | +1, -1
        let mid = digits[imid];
        let mut saved_diff = 99999;
        let mut saved_num = 0;

        for ix in vec![imid as i32 - 1, imid as i32] {
            let mut newdigits = digits.clone();
            for inc in vec![-1, 1] {
                if ix >= 0 {
                    let ix = ix as usize;
                    if newdigits[ix] + inc >= 0 && newdigits[ix] + inc <= 9 {
                        newdigits[ix] += inc;

                        Self::reflect(&mut newdigits);

                        let newnum = Self::to_num(&newdigits);

                        let diff = (orignum - newnum).abs();
                        if orignum != newnum && diff < saved_diff {
                            saved_diff = diff;
                            saved_num = newnum;
                        }
                    }
                }
            }
        }
        dbg!(saved_num);
        saved_num.to_string()
    }
}

#[test]
fn test() {
    use Solution;

    Solution::nearest_palindromic("12345".to_string());

    /*
    assert_eq!(Solution::nearest_palindromic("1".to_string()), "0");
    assert_eq!(Solution::nearest_palindromic("0".to_string()), "1");
    assert_eq!(Solution::nearest_palindromic("121".to_string()), "111");
    assert_eq!(Solution::nearest_palindromic("101".to_string()), "111");
    */
}
