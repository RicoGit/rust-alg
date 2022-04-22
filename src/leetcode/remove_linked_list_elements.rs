//! 203. Remove Linked List Elements

use std::ops::DerefMut;

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
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy_head = ListNode { val: 0, next: None };
        let mut cursor = &mut dummy_head;

        while let Some(mut node) = head {
            head = std::mem::replace(&mut node.next, None);

            if node.val != val {
                cursor.next = Some(node); // add next
                cursor = cursor.next.as_mut().unwrap(); // move to next
            }
        }

        dummy_head.next
    }
}

struct Solution;
