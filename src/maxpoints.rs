// Ref: https://leetcode.com/problems/max-points-on-a-line/

use std::cmp::Eq;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::ops::Add;
use std::ops::Mul;

fn gcd(a: i128, b: i128) -> i128 {
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

fn lcm(a: i128, b: i128) -> i128 {
    if (a == 0) || (b == 0) {
        0
    } else {
        ((a * b) / gcd(a, b))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Fraction {
    num: i128,
    den: i128,
}

fn ifelse<T: Copy>(cond: bool, thenval: T, elseval: T) -> T {
    if (cond) {
        thenval
    } else {
        elseval
    }
}

impl Fraction {
    fn new(num: i128, den: i128) -> Fraction {
        let num = num as i128;
        let den = den as i128;

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
    a: Option<i128>, // x-offset: for vertical lines, `m` == `b` == None and `a` denotes the x-offset
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
    fn new(p1: &Vec<i32>, p2: &Vec<i32>) -> Line {

        //println!("{:?} {:?}", p1, p2);
        let (x1, y1, x2, y2) = (p1[0] as i128, p1[1] as i128, p2[0] as i128, p2[1] as i128);
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
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Line2 {
    m: f64,
    b: f64,
    a: f64
}

impl Line2 {
    fn new(p1: &Vec<i32>, p2: &Vec<i32>) -> Line2 {
        //println!("{:?} {:?}", p1, p2);
        let (x1, y1, x2, y2) = (p1[0] as f64, p1[1] as f64, p2[0] as f64, p2[1] as f64);
        let m = if (x1 == x2) { 0.0 } else { (y2 - y1) / (x2 - x1) };
        let b = if (x1 == x2) {
            0.0
        } else {
            let b_num = ((x2 - x1) * y1) - ((y2 - y1) * x1);
            let b_den = x2 - x1;
            b_num / b_den
        };
        let a = if (x1 == x2) { x1 } else { 0.0 };
        Line2 { m, b, a }
    }
}

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let mut lines = HashMap::new();

        // Determine all lines on the plane
        for (i, p1) in points.iter().enumerate() {
            for (j, p2) in points.iter().enumerate() {
                if (j <= i) {
                    continue;
                }
                let line = Line::new(p1, p2);
                let set = lines.entry(line).or_insert(HashSet::new());
                set.insert(i);
                set.insert(j);
            }
        }

        if points.len() <= 1 {
            points.len() as i32
        } else {
            let retval = lines.iter().map(|(_,set)| set.len()).max();
            retval.unwrap() as i32
        }
    }
}

#[test]
fn test() {
    let points = vec![[1, 1], [3, 2], [5, 3], [4, 1], [2, 3], [1, 4]];
    //let points = vec![[4, 0], [4, 1], [4, -1], [4, 8]];
    let points = vec![[1, 1], [3, 2], [5, 3], [4, 1], [2, 3], [1, 4]];
    let points = vec![[0,0],[1,65536],[65536,0], [0,0]];
    //let points = vec![[0,0],[1,0],[1,0],[2,0]];

    let points: Vec<Vec<i32>> = points.iter().map(|point| vec![point[0], point[1]]).collect();

    println!("\n");
    dbg!(Solution::max_points(points));
    println!("\n");
}

struct Solution {}

