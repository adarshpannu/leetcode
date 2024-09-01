impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1.as_ref();
        let mut l2 = l2.as_ref();

        let mut digit_sums = Vec::with_capacity(101);
        let mut carry = 0;

        while l1.is_some() || l2.is_some() {
            let mut sum = 0;
            if let Some(lst_node1) = l1 {
                sum = lst_node1.val;
                l1 = lst_node1.next.as_ref();
            }
            if let Some(lst_node2) = l2 {
                //println!("{} + {}", sum, lst_node2.val);
                sum += lst_node2.val;
                l2 = lst_node2.next.as_ref();
            }
            sum += carry;
            carry = sum / 10;
            sum = sum % 10;

            //println!("digit = {} carry = {}", sum, carry);
            digit_sums.push(sum);
        }
        if carry > 0 {
            digit_sums.push(1);
        }
        Solution::digits_to_list(&digit_sums)
    }

    fn digits_to_list(v: &[i32]) -> Option<Box<ListNode>> {
        if v.len() == 0 {
            None
        } else {
            let mut curnode = None;
            for &e in v.iter().rev() {
                curnode = Some(Box::new(ListNode { val: e, next: curnode }))
            }
            curnode
        }
    }

    pub fn numstr_to_list(numstr: &str) -> Option<Box<ListNode>> {
        let mut curnode = None;

        for &e in numstr.as_bytes().iter() {
            let val = e - ('0' as u8);
            curnode = Some(Box::new(ListNode { val: val as i32, next: curnode }))
        }
        curnode
    }

    pub fn list_to_num(lst: Option<&Box<ListNode>>) -> i32 {
        let mut place_multiplier = 1;
        let mut num = 0;
        let mut lst = lst;
        while let Some(lst_node) = lst {
            num += (lst_node.val * place_multiplier);
            place_multiplier *= 10;
            lst = lst_node.next.as_ref();
        }
        num
    }
}

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

struct Solution {}

use rand::Rng; // 0.8.5

#[test]
pub fn lc2_add_two_numbers() {
    for _ in 1..=100000 {
        let n1 = rand::thread_rng().gen_range(1..100000);
        let n1str = n1.to_string();
        let n2 = rand::thread_rng().gen_range(1..100000);
        let n2str = n2.to_string();

        //let (n1, n1str) = (43, "43");
        //let (n2, n2str) = (98, "98");
        //println!(r"{n1}, {n2}");

        let l1 = Solution::numstr_to_list(&n1str);
        let l2 = Solution::numstr_to_list(&n2str);
        //dbg!(&l1);
        //dbg!(&l2);

        let lst = Solution::add_two_numbers(l1, l2);
        //dbg!(&lst);

        let sum = Solution::list_to_num(lst.as_ref());
        //dbg!(&sum);
        //dbg!(n1 + n2);

        assert!(n1 + n2 == sum)
    }
}
