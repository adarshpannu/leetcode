struct Solution;

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

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn traverse(node: Option<&Rc<RefCell<TreeNode>>>, cursum: i32) -> i32 {
        let mut retsum = cursum;
        if let Some(node) = node {
            if let Some(left) = node.borrow().left.as_ref() {
                if left.borrow().left.as_ref().is_none() && left.borrow().right.as_ref().is_none() {
                    retsum += left.borrow().val
                } else {
                    retsum += Self::traverse(Some(left), cursum)
                }
            }
            if let Some(right) = node.borrow().right.as_ref() {
                retsum += Self::traverse(Some(right), cursum)
            }
        }
        retsum
    }

    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::traverse(root.as_ref(), 0)
    }
}

#[test]
fn test() {
    use Solution;
}
