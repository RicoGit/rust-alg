//! 2. Add Two Numbers

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

impl Solution {
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = ListNode::new(0);
        let mut cursor = &mut dummy_head;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() {
            let n1 = l1.unwrap_or(Box::new(ListNode::new(0)));
            let n2 = l2.unwrap_or(Box::new(ListNode::new(0)));
            let sum = carry + n1.val + n2.val;
            carry = sum / 10;
            cursor.next = Some(Box::new(ListNode::new(sum % 10)));

            l1 = n1.next;
            l2 = n2.next;
            cursor = cursor.next.as_mut().unwrap()
        }
        if carry == 1 {
            cursor.next = Some(Box::new(ListNode::new(carry)))
        }

        dummy_head.next
    }
}

struct Solution;