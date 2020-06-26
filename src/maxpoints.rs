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

fn find_n_for_nc2(nc2: i32) -> i32 {
    let mut n = 2;
    let mut calnc2 = 1;
    
    loop {
        if (nc2 == calnc2) {
            return n;
        }
        if (n == 100) {
            panic!("Runaway loop? nc3 = {}", nc2);
        }
        n = n + 1;
        calnc2 = n * calnc2 / (n - 2);
    }
}

#[test]
fn test_nc2() {
    dbg!(find_n_for_nc2(1));
}

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {

        let mut points_grouped = HashMap::new();
        for p in &points {
            let mut count = points_grouped.entry(p).or_insert(0);
            *count += 1;
        }

        //println!("points_grouped = {:?}", points_grouped);

        let mut lines = HashMap::new();

        // Determine all lines on the plane
        for (i, (&p1, icount)) in points_grouped.iter().enumerate() {
            for (j, (&p2, jcount)) in points_grouped.iter().enumerate() {
                if (j <= i) {
                    continue;
                }
                let line = Line::new(p1, p2);
                let line_counts = (0, 0);
                let mut count = lines.entry(line).or_insert(line_counts);
                count.0 += 1;
                count.1 += (icount + jcount);
            }
        }

        //println!("lines = {:?}", lines);
        //dbg!(points.len());

        if points.len() <= 2 || lines.len() == 0 {
            points.len() as i32
        } else {
            let retval = lines.iter().map(|(_,&count)| count.1 / (find_n_for_nc2(count.0) - 1)).max().unwrap();
            retval
        }
    }
}

#[test]
fn test() {
    let points = vec![[0,0],   [1,0],[1,0],  [2,0],   [4,0],   [8,0]];

    let points: Vec<Vec<i32>> = points.iter().map(|point| vec![point[0], point[1]]).collect();

    println!("\n");
    dbg!(Solution::max_points(points));
    println!("\n");
}


struct Solution {}

