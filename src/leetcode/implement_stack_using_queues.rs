//! 225. Implement Stack using Queues

use std::collections::LinkedList;

#[derive(Debug)]
struct MyStack {
    q: LinkedList<i32>,
}

// queue <]
// queue <1, 2, 4]
// stack 3 5
impl MyStack {
    fn new() -> Self {
        MyStack {
            q: LinkedList::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.q.push_back(x);
        let mut size = self.q.len();

        while size > 1 {
            let el = self.q.pop_front().unwrap();
            self.q.push_back(el);
            size -= 1;
        }
    }

    fn pop(&mut self) -> i32 {
        self.q.pop_front().unwrap()
    }

    fn top(&mut self) -> i32 {
        *self.q.front().unwrap()
    }

    fn empty(&self) -> bool {
        self.q.is_empty()
    }
}

struct Solution;
