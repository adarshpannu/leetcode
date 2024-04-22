use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    // Returns (sum, level) pair
    pub fn deepest_leaves_sum_n_level(
        node: Option<Rc<RefCell<TreeNode>>>,
        curlevel: i32,
    ) -> (i32, i32) {
        if let Some(node) = node {
            let node = &*node.borrow();
            let (leftsum, leftlevel) =
                Self::deepest_leaves_sum_n_level(node.left.clone(), curlevel + 1);
            let (rightsum, rightlevel) =
                Self::deepest_leaves_sum_n_level(node.right.clone(), curlevel + 1);

            let retlevel = curlevel.max(leftlevel).max(rightlevel);
            let mut retsum = 0;
            if curlevel == retlevel {
                retsum += node.val
            };
            if leftlevel == retlevel {
                retsum += leftsum
            };
            if rightlevel == retlevel {
                retsum += rightsum
            };
            return (retsum, retlevel);
        } else {
            return (0, 0);
        }
    }

    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::deepest_leaves_sum_n_level(root.clone(), 0).0
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
