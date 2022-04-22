//! 1721. Swapping Nodes in a Linked List

use std::mem::swap;

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

    fn new_list(values: &[i32]) -> Option<Box<Self>> {
        if values.len() == 0 {
            return None;
        }
        ListNode::new_node(&values, ListNode::new_list(&values[1..]))
    }

    fn new_node(values: &[i32], next: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        values
            .first()
            .map(|val| Box::new(ListNode { val: *val, next }))
    }
}

impl Solution {
    pub fn swap_nodes(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let k = (k - 1) as usize;

        let mut array = Solution::copy_to_array(head.expect("Empty linked list is not allowed"));
        let second_idx = array.len() - 1 - k;

        array.swap(k, second_idx);

        Solution::copy_from_array(array)
    }

    fn copy_to_array(head: Box<ListNode>) -> Vec<i32> {
        let mut arr = vec![];
        let mut current = head;
        arr.push(current.val);
        while let Some(node) = current.next {
            arr.push(node.val);
            current = node;
        }
        arr
    }

    fn copy_from_array(arr: Vec<i32>) -> Option<Box<ListNode>> {
        let mut prev = None;
        for mut val in arr.into_iter().rev() {
            prev = Some(Box::new(ListNode { val, next: prev }));
        }
        prev
    }
}

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = ListNode::new_list(&vec![1, 2, 3, 4, 5]);
        let expected = ListNode::new_list(&vec![1, 4, 3, 2, 5]);
        assert_eq!(Solution::swap_nodes(input, 2), expected);
    }
}
