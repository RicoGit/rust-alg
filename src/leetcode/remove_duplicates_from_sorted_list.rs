//! 83. Remove Duplicates from Sorted List

use crate::leetcode::increase_order_search_tree::TreeNode;

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

type Node = Option<Box<ListNode>>;

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = ListNode {
            val: i32::MIN,
            next: None,
        };
        let mut cursor = &mut dummy_head;

        while let Some(node) = head {
            head = node.next;

            if cursor.val != node.val {
                cursor.next = Some(Box::new(ListNode::new(node.val))); // add next
                cursor = cursor.next.as_mut().unwrap(); // move cursor
            }
        }
        cursor.next = None;

        dummy_head.next
    }
}

struct Solution;
