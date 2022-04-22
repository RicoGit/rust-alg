//! 876. Middle of the Linked List

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
    // iterative version
    pub fn middle_node_iterative(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut mid_node: Box<ListNode> = head.clone().unwrap();
        let mut current_node: Box<ListNode> = head.unwrap();
        let mut odd = true;

        loop {
            match &current_node.next {
                Some(next) => {
                    current_node = next.clone();
                }
                None => return Some(mid_node),
            }
            if odd && mid_node.next.is_some() {
                mid_node = mid_node.next.unwrap();
            }
            odd = !odd;
        }
    }

    // recursive version
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Solution::next_node(head.clone(), head, false)
    }

    fn next_node(
        start: Option<Box<ListNode>>,
        mid_node: Option<Box<ListNode>>,
        odd: bool,
    ) -> Option<Box<ListNode>> {
        match start {
            None => return mid_node,
            node @ Some(_) => {
                let next_node = node.clone().and_then(|n| n.next);
                let not_end = mid_node.as_ref().is_some();
                Solution::next_node(
                    next_node,
                    if odd && not_end {
                        mid_node.and_then(|mid| mid.next)
                    } else {
                        mid_node
                    },
                    !odd,
                )
            }
        }
    }
}

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = ListNode::new_list(&vec![1, 2, 3, 4, 5]);
        let expected = ListNode::new_list(&vec![3, 4, 5]);
        let result = Solution::middle_node(input);
        assert_eq!(result, expected)
    }

    #[test]
    fn test2() {
        let input = ListNode::new_list(&vec![1, 2, 3, 4, 5, 6]);
        let expected = ListNode::new_list(&vec![4, 5, 6]);
        let result = Solution::middle_node(input);
        assert_eq!(result, expected)
    }
}
