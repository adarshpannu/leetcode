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

impl Solution {
    /*
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(rc) = root {
            let tn = rc.borrow();
            let value = tn.val;
            let mut queue = vec![tn];

            loop {
                if let Some(node) = queue.pop() {
                    if node.val != value {
                        return false;
                    }
                    if let Some(ref rc_left) = node.left {
                        let refcell = &**rc_left;
                        let node = refcell.borrow();
                        queue.push(node);
                    }
                } else {
                    break;
                }
            }
        }
        true
    }
    */
}

struct Solution;

#[test]
fn test() {
    let left = TreeNode::new(-10);
    let right = TreeNode::new(25);

    let mut parent = TreeNode::new(1);
    parent.left = Some(Rc::new(RefCell::new(left)));
    parent.right = Some(Rc::new(RefCell::new(right)));

    //assert_eq!(Solution::is_unival_tree(Some(Rc::new(RefCell::new(parent)))), false);

    let tn = Some(Rc::new(RefCell::new(parent)));
    if let Some(node_opt) = tn {
        let node = &*node_opt.borrow();
        //let left = node.left.as_ref().unwrap();

    }
    dbg!(tn);

}
