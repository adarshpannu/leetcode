// Ref: https://leetcode.com/problems/non-decreasing-array/

use std::collections::HashSet;

impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut nchanges = 0;
        let mut old_set: HashSet<usize> = HashSet::new();

        for (ix, win) in nums.windows(3).enumerate() {
            let fix = Self::fix(ix, win);
            //println!("window: {:?}, fix = {:?}", win, fix);
            match fix {
                None => {
                    return false;
                }
                Some(set) => {
                    if (set.len() > 0) {
                        // needs a fix?
                        let set_x: HashSet<&usize> = set.intersection(&old_set).collect();
                        if set_x.len() == 0 {
                            nchanges += 1;
                        }
                    }
                    old_set = set;
                }
            }
        }
        nchanges <= 1
    }

    pub fn fix(offset: usize, arr: &[i32]) -> Option<HashSet<usize>> {
        let mut set: HashSet<usize> = HashSet::new();
        let ab = (arr[0] <= arr[1]) as i32;
        let bc = (arr[1] <= arr[2]) as i32;
        let ac = (arr[0] <= arr[2]) as i32;
        match (ab, bc, ac) {
            (1, 1, 1) => {} // 10 20 30
            (1, 1, 0) => {} // not possible
            (1, 0, 1) => {
                // 10 30 20
                set.insert(offset + 1);
                set.insert(offset + 2);
            }
            (1, 0, 0) => {
                set.insert(offset + 2);
            } // 20 30 10
            (0, 0, 0) => {
                return None;
            } // 30 20 10 (dqed)
            (0, 0, 1) => {} // not possible
            (0, 1, 0) => {
                set.insert(offset + 0);
            } // 30 10 20
            (0, 1, 1) => {
                // 20 10 30
                set.insert(offset + 0);
                set.insert(offset + 1);
            }
            _ => {}
        }
        Some(set)
    }
}

struct Solution;

#[test]
fn test() {
    assert_eq!(Solution::check_possibility(vec!(4, 2, 1)), false);
    assert_eq!(Solution::check_possibility(vec!(4, 2, 3)), true);
    assert_eq!(Solution::check_possibility(vec!(3, 4, 2, 3)), false);
    assert_eq!(Solution::check_possibility(vec!(-1, -1, 0, 4, 2, 3)), true);
    assert_eq!(Solution::check_possibility(vec!(2,1,3,5,4)), false);
    assert_eq!(Solution::check_possibility(vec!(-1, 4, 2, 3)), true);
}
