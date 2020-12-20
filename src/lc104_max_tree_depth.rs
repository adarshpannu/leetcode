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
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::max_depth0(root.as_ref())
    }

    pub fn max_depth0(root: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(rcnode) = root {
            return 1 + Solution::max_depth0(rcnode.borrow().left.as_ref())
                .max(Solution::max_depth0(rcnode.borrow().right.as_ref()));
        } else {
            return 0;
        }
    }
}

#[test]
fn test() {}
