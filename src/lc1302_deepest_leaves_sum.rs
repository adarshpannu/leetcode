use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {

            let mut stack = Vec::with_capacity(512);
            stack.push((node, 0));

            let mut max_level = 0;
            let mut sum = 0;

            while let Some((curnode, cur_level)) = stack.pop() {
                let curnode: &TreeNode = &*curnode.borrow();
                let mut has_right = false;
                if let Some(rnode) = &curnode.right {
                    stack.push((Rc::clone(rnode), cur_level + 1));
                    has_right = true;
                }

                if let Some(lnode) = &curnode.left {
                    stack.push((Rc::clone(lnode), cur_level + 1));
                } else if !has_right {
                    // Leaf node
                    if cur_level > max_level {
                        (sum, max_level) = (curnode.val, cur_level)
                    } else if cur_level == max_level {
                        sum += curnode.val;
                    }
                }
            }
            sum
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
