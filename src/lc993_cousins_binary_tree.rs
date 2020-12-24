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

struct FindResult {
    parent: i32,
    depth: i32,
}

impl Solution {
    fn find(
        val: i32,
        root: Option<&Rc<RefCell<TreeNode>>>,
        parent: i32,
        depth: i32,
    ) -> Option<FindResult> {
        if let Some(tn) = root {
            let tn = tn.borrow();
            if tn.val == val {
                return Some(FindResult { parent, depth });
            } else if let Some(find_result) = Self::find(val, tn.left.as_ref(), tn.val, depth + 1) {
                return Some(find_result);
            } else if let Some(find_result) = Self::find(val, tn.right.as_ref(), tn.val, depth + 1) {
                return Some(find_result);
            }
        }
        None
    }

    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let x_find = Self::find(x, root.as_ref(), -1, 0);
        let y_find = Self::find(y, root.as_ref(), -1, 0);
        return if let (Some(x_find), Some(y_find)) = (x_find, y_find) {
            x_find.parent != y_find.parent && x_find.depth == y_find.depth
        } else {
            false
        };
    }
}

#[test]
fn test() {
    use Solution;
}
