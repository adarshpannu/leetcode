// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut node = head.as_ref();
        let mut retval = 0;

        while let Some(ln) = node {
            node = ln.next.as_ref();
            retval = (retval * 2) + ln.val;
        }
        retval
    }
}

#[test]
fn test1() {
    let ln = ListNode::new(1);
    let ln = ListNode {
        val: 0,
        next: Some(Box::new(ln)),
    };
    let ln = ListNode {
        val: 1,
        next: Some(Box::new(ln)),
    };

    println!("dec value: {}", Solution::get_decimal_value(Some(Box::new(ln))));
}
