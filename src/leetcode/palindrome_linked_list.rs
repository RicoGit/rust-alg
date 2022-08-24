//! 234. Palindrome Linked List
//!
//! Really hard to understand runtime: O(n) space: O(1)



// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return true
        }

        // calculate size of list
        let mut cursor = head.as_ref();
        let mut size = 0;
        while let Some(node) = cursor {
            cursor = node.next.as_ref();
            size += 1;
        }

        // revert first half of list and compare with rest
        let mut cursor = head;
        let mut next = None;
        let mut reverted = None;
        let mut half_size = size / 2 + 1;
        while cursor.is_some() {
            half_size -= 1;
            if half_size == 0 {
                // when we reach middle of the list, compare 2 parts
                return if size % 2 == 0 { reverted == cursor } else { reverted == cursor.as_ref().unwrap().next }
            }
            // safe next node
            next = cursor.as_mut().unwrap().next.take();
            // assign prev Node as next (change link direction)
            cursor.as_mut().unwrap().next = reverted;
            // update last node in reverted linked list
            reverted = cursor;
            // update cursor
            cursor = next;
        }

        false
    }
}

struct Solution;
