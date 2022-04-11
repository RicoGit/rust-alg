//! 21. Merge Two Sorted Lists


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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        match (list1, list2) {
            (None, None) => None,
            (None, node @ Some(_)) => node,
            (node @ Some(_), None) => node,
            (Some(mut node1), Some(mut node2)) => {
                if node1.val < node2.val {
                    node1.next = Self::merge_two_lists(node1.next, Some(node2));
                    Some(node1)
                } else {
                    node2.next = Self::merge_two_lists(Some(node1), node2.next);
                    Some(node2)
                }
            }

        }


    }
}

struct Solution;
