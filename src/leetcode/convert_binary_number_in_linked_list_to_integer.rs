//! 1290. Convert Binary Number in a Linked List to Integer

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
impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut cursor = head;
        let mut result = 0;
        while let Some(node) = cursor {
            cursor = node.next;
            result <<= 1;
            result |= node.val
        }
        result
    }
}

struct Solution;
