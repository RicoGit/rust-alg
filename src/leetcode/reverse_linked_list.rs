//! 206. Reverse Linked List

use std::mem;

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
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::reverse(head, None)
    }

    fn reverse(
        head: Option<Box<ListNode>>,
        reversed: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match head {
            None => reversed,
            Some(mut node) => {
                let new_head = mem::replace(&mut node.next, reversed);
                Self::reverse(new_head, Some(node))
            }
        }
    }
}

struct Solution;
