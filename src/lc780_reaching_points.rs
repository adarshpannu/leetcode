use std::collections::VecDeque;

fn is_even(i: i32) -> bool {
    i % 2 == 0
}

fn is_odd(i: i32) -> bool {
    !is_even(i)
}

impl Solution {
    pub fn reaching_points(sx: i32, sy: i32, tx: i32, ty: i32) -> bool {
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        let mut n_considered = 0;

        q.push_front((sx, sy));
        while let Some((sx, sy)) = q.pop_front() {
            n_considered += 1;
            println!("Candidate: ({}, {})", sx, sy);

            if (sx, sy) == (tx, ty) {
                dbg!(n_considered);
                return true;
            }
            for (ex, ey) in Self::transform(sx, sy).iter() {
                if !Self::prune(*ex, *ey, tx, ty) {
                    q.push_back((*ex, *ey));
                }
            }
        }
        dbg!(n_considered);
        false
    }

    fn transform(sx: i32, sy: i32) -> [(i32, i32); 2] {
        [(sx, (sx + sy)), ((sx + sy), sy)]
    }

    fn prune(sx: i32, sy: i32, tx: i32, ty: i32) -> bool {
        if is_even(sx) && is_even(sy) && (is_odd(tx) || is_odd(ty)) {
            return true;
        }
        if (sx <= tx) && (sy <= ty) && (sx + sy <= tx + ty) {
            return false;
        } else {
            return true;
        }
    }
}

struct Solution;

#[test]
fn test() {
    //assert_eq!(Solution::reaching_points(1, 1, 3, 5), true);
    //assert_eq!(Solution::reaching_points(1, 1, 1, 1), true);
    //assert_eq!(Solution::reaching_points(35, 13, 455955547, 420098884), true);
    assert_eq!(Solution::reaching_points(1, 1, 998, 999), true);
}
