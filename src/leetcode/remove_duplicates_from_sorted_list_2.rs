//! 82. Remove Duplicates from Sorted List II

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
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_list = ListNode {
            val: i32::MIN,
            next: None,
        };

        let mut new_list_cursor = &mut new_list;
        let mut candidate = ListNode::new(i32::MIN);
        let mut good_candidate = false;

        while let Some(node) = head {
            head = node.next;

            if candidate.val != node.val {
                // value was updated,
                if good_candidate {
                    // if candidate was without duplicates add to result
                    new_list_cursor.next = Some(Box::new(candidate));
                    new_list_cursor = new_list_cursor.next.as_mut().unwrap();
                }
                good_candidate = true;
                candidate = ListNode::new(node.val);
            } else {
                good_candidate = false;
            }
        }

        if good_candidate {
            new_list_cursor.next = Some(Box::new(candidate));
        }

        new_list.next
    }
}

struct Solution;
