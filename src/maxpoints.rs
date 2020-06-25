// Ref: https://leetcode.com/problems/max-points-on-a-line/

struct Solution {}

use std::cmp::Eq;
use std::collections::HashMap;
use std::fmt;

fn gcd(a: u32, b: u32) -> u32 {
    let mut remainder = 0u32;
    let (mut a, mut b) = (a, b);
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

fn lcm(a: u32, b: u32) -> u32 {
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
    num: u32,
    den: u32,
    is_negative: bool,
}

impl Fraction {
    fn new(num: i32, den: i32) -> Fraction {
        let is_negative = (num < 0) ^ (den < 0);
        let num: u32 = num.abs() as u32;
        let den: u32 = den.abs() as u32;

        if (num == 0) || (den == 0) {
            Fraction { num, den, is_negative }
        } else {
            let lcm = lcm(num, den);
            Fraction { num: lcm / den, den: lcm / num, is_negative }
        }
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = if self.is_negative { "-" } else { "" };
        write!(f, "{}{:?}/{:?}", sign, self.num, self.den)?;
        Ok(())
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
            false 
        }
    }
}

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        0
    }
}

#[test]
fn test() {
    let points = vec![[1, 1], [3, 2], [5, 3], [4, 1], [2, 3], [1, 4]];
    //let points = vec![[1, 1], [2, 2], [3, 3], [4, 4]];
    let points = vec![[4, 0], [4, 1], [4, -1], [4, 8]];
    let points: Vec<Vec<i32>> = points.iter().map(|point| vec![point[0], point[1]]).collect();
    let mut lines = HashMap::new();

    let mut count = 0;

    for (i, p1) in points.iter().enumerate() {
        for (j, p2) in points.iter().enumerate() {
            if j <= i {
                continue;
            }
            for (k, p3) in points.iter().enumerate() {
                if k <= j {
                    continue;
                }
                let line12 = Line::new_from_pairs(p1, p2);
                let line23 = Line::new_from_pairs(p2, p3);

                count += 1;
                if line12 == line23 {
                    println!(
                        "EQ: p1 = {:?}, p2 = {:?}, p3 = {:?}, line12 = {}, line23 = {}",
                        p1, p2, p3, line12, line23
                    );
                    lines.insert(line12, 1);
                }
            }
        }
    }

    if lines.len() == 0 {
        // No lines with 2+ points
    } else {
        // 
    }
    println!("count = {}", count);
}
