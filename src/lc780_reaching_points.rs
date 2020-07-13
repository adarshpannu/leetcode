use std::collections::VecDeque;

impl Solution {
    pub fn reaching_points(sx: i32, sy: i32, tx: i32, ty: i32) -> bool {
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        q.push_front((sx, sy));
        while let Some((sx, sy)) = q.pop_front() {
            //println!("Candidate: ({}, {})", sx, sy);
            if (sx, sy) == (tx, ty) {
                return true;
            }
            for (ex, ey) in Self::transform(sx, sy).iter() {
                if (*ex <= tx) && (*ey <= ty) && (*ex + *ey <= tx + ty) {
                    q.push_back((*ex, *ey));
                }
            }
        }
        false
    }

    fn transform(sx: i32, sy: i32) -> [(i32, i32); 2] {
        [(sx, (sx + sy)), ((sx + sy), sy)]
    }
}

struct Solution;

#[test]
fn test() {
    //assert_eq!(Solution::reaching_points(1, 1, 3, 5), true);
    //assert_eq!(Solution::reaching_points(1, 1, 1, 1), true);
    assert_eq!(Solution::reaching_points(35, 13, 455955547, 420098884), true);

}
