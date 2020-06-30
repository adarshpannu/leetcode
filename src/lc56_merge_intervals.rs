#![allow(warnings)]

// Ref: https://leetcode.com/problems/merge-intervals/

/*
Given a collection of intervals, merge all overlapping intervals.

Example 1:
Input: [[1,3],[2,6],[8,10],[15,18]]
Output: [[1,6],[8,10],[15,18]]
Explanation: Since intervals [1,3] and [2,6] overlaps, merge them into [1,6].

Example 2:
Input: [[1,4],[4,5]]
Output: [[1,5]]
Explanation: Intervals [1,4] and [4,5] are considered overlapping.
*/

type Interval = [i32; 2];

fn merge(i1: &[i32; 2], i2: &[i32; 2]) -> Option<Interval> {
    assert!(i1[0] <= i1[1]);
    assert!(i2[0] <= i2[1]);

    if i1[0] <= i2[0] && i2[0] <= i1[1] {
        return Some([i1[0], i2[1]]);
    } else if i2[0] <= i1[0] && i1[0] <= i2[1] {
        return Some([i2[0], i1[1]]);
    } else {
        return None;
    }
}

#[test]
fn unit_test_1() {
    assert_eq!(merge(&[1, 4], &[2, 8]), Some([1, 8]));
    assert_eq!(merge(&[2, 8], &[1, 4]), Some([1, 8]));
    assert_eq!(merge(&[2, 8], &[8, 20]), Some([2, 20]));
    assert_eq!(merge(&[8, 20], &[2, 8]), Some([2, 20]));
}

#[test]
fn run() {
    let intervals = [[1, 3], [2, 6], [8, 10], [15, 18]];
    //let intervals = [[1, 4], [4, 5]];

    for (i, ipair) in intervals.iter().enumerate() {
        for (j, jpair) in intervals.iter().enumerate() {
            if j > i {
                let merged = merge(ipair, jpair);
                match merged {
                    Some(mpair) => println!("{:?} <- {:?} + {:?}", mpair, ipair, jpair),
                    _ => {}
                }
            }
        }
    }
}
