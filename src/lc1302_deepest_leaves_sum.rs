use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    // Returns (sum, level) pair
    pub fn deepest_leaves_sum_n_level(node: &TreeNode, curlevel: i32) -> (i32, i32) {
        let (mut retsum, mut retlevel) = if let Some(node) = &node.left {
            Self::deepest_leaves_sum_n_level(&*node.borrow(), curlevel + 1)
        } else {
            (node.val, curlevel + 1)
        };

        if let Some(node) = &node.right {
            let (rsum, rlevel) = Self::deepest_leaves_sum_n_level(&*node.borrow(), curlevel + 1);
            if rlevel > retlevel {
                (retsum, retlevel) = (rsum, rlevel)
            } else if rlevel == retlevel {
                retsum += rsum
            }
        }

        return (retsum, retlevel);
    }

    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            Self::deepest_leaves_sum_n_level(&*node.borrow(), 0).0
        } else {
            0
        }
    }
}

#[test]
pub fn test() {
    use Solution;
}

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
        TreeNode { val, left: None, right: None }
    }
}
