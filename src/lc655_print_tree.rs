// https://leetcode.com/problems/print-binary-tree/

// Definition for a binary tree node.

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
        vec!(vec!(String::from("")))
    }
    INCOMPLETE
}

#[test]
fn test() {
    let left = TreeNode::new(1);
    let right = TreeNode::new(2);

    let mut parent = TreeNode::new(3);
    parent.left = Some(Rc::new(RefCell::new(left)));
    parent.right = Some(Rc::new(RefCell::new(right)));

    let parent = Some(Rc::new(RefCell::new(parent)));

    dbg!(&parent);

    Solution::print_tree(parent);
}

struct Solution;


