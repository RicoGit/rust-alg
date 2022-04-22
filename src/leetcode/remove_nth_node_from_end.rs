//! 19. Remove Nth Node From End of List

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

use std::ops::DerefMut;
type Node = Option<Box<ListNode>>;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let res = Solution::get_nth_node(&mut head, n as usize).map(|node| unsafe {
            let mut node = &mut *node;
            let next_to_nth_node = node.next.clone().and_then(|next| next.next);
            node.next = next_to_nth_node;
        });

        if res.is_none() {
            return head.unwrap().next;
        }
        head
    }

    /// Return pointer to Nth node from the end
    fn get_nth_node(head: &mut Node, n: usize) -> Option<*mut ListNode> {
        fn get_nth_node_rec(
            head: &Option<Box<ListNode>>,
            nth_node: Option<*mut ListNode>,
            iteration: usize,
            n: usize,
        ) -> Option<*mut ListNode> {
            match head {
                None => {
                    // we reach the end
                    return if iteration > n { nth_node } else { None };
                }
                Some(node) => {
                    if iteration <= n {
                        return get_nth_node_rec(&node.next, nth_node, iteration + 1, n);
                    } else {
                        unsafe {
                            let next_node_ptr: Option<*mut ListNode> = nth_node.and_then(|ptr| {
                                (*ptr).next.as_mut().map(|h| h.deref_mut() as *mut ListNode)
                            });
                            return get_nth_node_rec(&node.next, next_node_ptr, iteration + 1, n);
                        };
                    }
                }
            }
        }

        let ptr: Option<*mut ListNode> = head.as_mut().map(|h| h.deref_mut() as *mut ListNode);
        get_nth_node_rec(&head, ptr, 0, n)
    }
}

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_nth_node_test1() {
        let mut input = ListNode::new_list(&vec![1, 2, 3, 4, 5]);
        let expected = ListNode::new_list(&vec![3, 4, 5]);
        let result = Solution::get_nth_node(&mut input, 2);
        unsafe { assert_eq!(*(result.unwrap()), *expected.unwrap()) }
    }

    #[test]
    fn remove_nth_from_end_test1() {
        let input = ListNode::new_list(&vec![1, 2, 3, 4, 5]);
        let expected = ListNode::new_list(&vec![1, 2, 3, 5]);
        let result = Solution::remove_nth_from_end(input, 2);
        assert_eq!(result, expected)
    }

    #[test]
    fn remove_nth_from_end_test2() {
        let input = ListNode::new_list(&vec![1]);
        let result = Solution::remove_nth_from_end(input, 1);
        assert_eq!(result, None)
    }

    #[test]
    fn remove_nth_from_end_test3() {
        let input = ListNode::new_list(&vec![1, 2]);
        let expected = ListNode::new_list(&vec![2]);
        let result = Solution::remove_nth_from_end(input, 2);
        assert_eq!(result, expected)
    }
}
