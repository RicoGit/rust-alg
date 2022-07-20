//! 232. Implement Queue using Stacks

struct MyQueue {
    stack: Vec<i32>,
    rev: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        MyQueue {
            stack: vec![],
            rev: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        while let Some(el) = self.rev.pop() {
            self.stack.push(el);
        }
        self.stack.push(x);
        while let Some(el) = self.stack.pop() {
            self.rev.push(el);
        }
    }

    fn pop(&mut self) -> i32 {
        self.rev.pop().unwrap()
    }

    fn peek(&self) -> i32 {
        *self.rev.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.rev.is_empty()
    }
}

struct Solution;
