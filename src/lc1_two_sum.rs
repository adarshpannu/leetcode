// Ref: https://leetcode.com/problems/two-sum/

/*
Given an array of integers, return indices of the two numbers such that they add up to a specific target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

Given nums = [2, 7, 11, 15], target = 9,

*/

fn get_indices(arr: &[i32], expected_sum: i32) -> Option<[usize; 2]> {
    for (i, ival) in arr.iter().enumerate() {
        for (j, jval) in arr.iter().enumerate() {
            if j > i {
                if (ival + jval == expected_sum) {
                    return Some([i, j]);
                }
            }
        }
    }
    None
}

#[test]
fn run() {
    let arr = [7, 11, -2, 15, 5];

    println!("indices of {:?} are {:?}", arr, get_indices(&arr, 9));

}