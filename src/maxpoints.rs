// Ref: https://leetcode.com/problems/max-points-on-a-line/

use std::cmp::Eq;
use std::collections::HashMap;
use std::fmt;
use std::ops::Add;
use std::ops::Mul;

fn gcd(a: i32, b: i32) -> i32 {
    let mut remainder = 0;
    let (mut a, mut b) = (a, b);

    if (a < 1 || b < 1) {
        panic!("Bad arguments to gcd()");
    }

    loop {
        remainder = a % b;
        a = b;
        b = remainder;
        if b == 0 {
            break;
        }
    }
    a
}

fn lcm(a: i32, b: i32) -> i32 {
    if (a == 0) || (b == 0) {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Fraction {
    num: i32,
    den: i32,
}

fn ifelse<T: Copy>(cond: bool, thenval: T, elseval: T) -> T {
    if (cond) {
        thenval
    } else {
        elseval
    }
}

impl Fraction {
    fn new(num: i32, den: i32) -> Fraction {
        assert!(den != 0);
        let sign = ifelse((num < 0) ^ (den < 0), -1, 1);
        let (num, den) = (num.abs(), den.abs());
        if (num == 0) {
            let den = 1; // 0/x normalized to 0/1
            Fraction { num, den }
        } else {
            let lcm = lcm(num, den);
            Fraction { num: lcm * sign / den, den: lcm / num }
        }
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = if self.num < 0 { "-" } else { "" };
        write!(f, "{}{:?}/{:?}", sign, self.num, self.den)?;
        Ok(())
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let num = (self.num * other.den) + (self.den * other.num);
        let den = (self.den * other.den);
        Self::new(num, den)
    }
}
impl Mul for Fraction {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let num = (self.num * other.num);
        let den = (self.den * other.den);
        Self::new(num, den)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Line {
    m: Option<Fraction>, // slope.
    b: Option<Fraction>, // y-offset.
    a: Option<i32>, // x-offset: for vertical lines, `m` == `b` == None and `a` denotes the x-offset
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.m.is_some() && self.b.is_some() {
            write!(f, "({},{})", self.m.unwrap(), self.b.unwrap())?;
        } else {
            write!(f, "|{}|", self.a.unwrap())?;
        }
        Ok(())
    }
}

impl Line {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Line {
        let m = if (x1 == x2) { None } else { Some(Fraction::new(y2 - y1, x2 - x1)) };
        let b = if (x1 == x2) {
            None
        } else {
            let b_num = ((x2 - x1) * y1) - ((y2 - y1) * x1);
            let b_den = x2 - x1;
            Some(Fraction::new(b_num, b_den))
        };
        let a = if (x1 == x2) { Some(x1) } else { None };
        Line { m, b, a }
    }

    fn new_from_pairs(p1: &Vec<i32>, p2: &Vec<i32>) -> Line {
        Line::new(p1[0], p1[1], p2[0], p2[1])
    }

    fn has_point(&self, p: &Vec<i32>) -> bool {
        let (x, y) = (p[0], p[1]);
        if let Some(a) = self.a {
            // Vertical line
            x == a
        } else {
            let y0 = self.m.unwrap() * Fraction::new(x, 1) + self.b.unwrap();
            y0 == Fraction::new(y, 1)
        }
    }

    pub fn on_a_line(p1: &Vec<i32>, p2: &Vec<i32>, p3: &Vec<i32>) -> Option<Line> {
        let line12 = Line::new_from_pairs(p1, p2);
        let line23 = Line::new_from_pairs(p2, p3);
        let line13 = Line::new_from_pairs(p1, p3);

        if (*p1 == *p2) && (*p2 == *p3) {
            panic!("All three points are identical? {:?} {:?} {:?}", *p1, *p2, *p3);
        }

        if *p1 == *p2 {
            Some(line13)
        } else if *p2 == *p3 {
            Some(line13)
        } else if *p1 == *p3 {
            Some(line12)
        } else if line12 == line23 {
            Some(line12)
        } else {
            None
        }
    }
}

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let mut lines = HashMap::new();

        for (i, p1) in points.iter().enumerate() {
            for (j, p2) in points.iter().enumerate() {
                if j <= i {
                    continue;
                }
                for (k, p3) in points.iter().enumerate() {
                    if k <= j {
                        continue;
                    }

                    if let Some(line) = Line::on_a_line(p1, p2, p3) {
                        let a = Line::on_a_line(p1, p2, p3);
                        
                        println!(
                            "EQ: p1 = {:?}, p2 = {:?}, p3 = {:?}, line = {}",
                            p1, p2, p3, line
                        );

                        // Insert new entry (if needed) and increment count
                        let count = lines.entry(line).or_insert(0);
                        *count += 1;
                    }
                }
            }
        }

        if points.len() <= 1 {
            // No lines with 2+ points
            points.len() as i32
        } else if lines.len() == 0 {
            2
        } else {
            let v = lines.values().into_iter().max().unwrap();
            find_n_for_nc3(*v)
        }
    }
}

#[test]
fn test() {
    let points = vec![[1, 1], [3, 2], [5, 3], [4, 1], [2, 3], [1, 4]];
    //let points = vec![[1, 1], [2, 2], [3, 3], [4, 4]];
    //let points = vec![[4, 0], [4, 1], [4, -1], [4, 8]];
    let points = vec![[1, 1], [3, 2], [5, 3], [4, 1], [2, 3], [1, 4]];
    let points = vec![[-435,-347],[-435,-347],[609,613],[-348,-267],[-174,-107],[87,133],[-87,-27],[-609,-507],[435,453],[-870,-747],[-783,-667],[0,53],[-174,-107],[783,773],[-261,-187],[-609,-507],[-261,-187],[-87,-27],[87,133],[783,773],[-783,-667],[-609,-507],[-435,-347],[783,773],[-870,-747],[87,133],[87,133],[870,853],[696,693],[0,53],[174,213],[-783,-667],[-609,-507],[261,293],[435,453],[261,293],[435,453]];

    let points: Vec<Vec<i32>> = points.iter().map(|point| vec![point[0], point[1]]).collect();

    dbg!(Solution::max_points(points));
}

fn find_n_for_nc3(nc3: i32) -> i32 {
    let mut calnc3 = 1;
    let mut n = 3;

    loop {
        if (nc3 == calnc3) {
            return n;
        }
        if (n == 100) {
            panic!("Runaway loop? nc3 = {}", nc3);
        }
        n = n + 1;
        calnc3 = n * calnc3 / (n - 3);
    }
}

#[test]
fn test_nc3() {
    dbg!(find_n_for_nc3(26235));
}

struct Solution {}
