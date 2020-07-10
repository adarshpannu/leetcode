// Ref: https://leetcode.com/problems/univalued-binary-tree/

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

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;

impl Solution {
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::check_tree(root.as_ref(), root.as_ref().map_or_else(|| -1, |node| node.borrow().val))
    }

    pub fn check_tree(root: Option<&Rc<RefCell<TreeNode>>>, val: i32) -> bool {
        if let Some(node) = root {
            if (node.borrow().val != val)
                || (Self::check_tree(node.borrow().left.as_ref(), val) == false)
                || (Self::check_tree(node.borrow().right.as_ref(), val) == false)
            {
                return false;
            }
        }
        return true;
    }


}

#[test]
fn test() {
    let left = TreeNode::new(-10);
    let right = TreeNode::new(25);

    let mut parent = TreeNode::new(1);
    parent.left = Some(Rc::new(RefCell::new(left)));
    parent.right = Some(Rc::new(RefCell::new(right)));

    let parent = Some(Rc::new(RefCell::new(parent)));

    assert_eq!(Solution::is_unival_tree(parent), false);
}

struct Solution;
