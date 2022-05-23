//! 622. Design Circular Queue

struct MyCircularQueue {
    q: Vec<i32>,
    size: usize,
    head: usize,
    tail: usize
}

impl MyCircularQueue {

    fn new(k: i32) -> Self {
        MyCircularQueue {
            q: vec![-1; k as usize],
            size: k as usize,
            head: 0,
            tail: 0
        }

    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.q[self.tail] = value;
            self.tail = (self.tail + 1) % self.size;
            true
        }
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.q[self.head] = -1;
            self.head = (self.head + 1) % self.size;
            true
        }
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.q[self.head]
        }
    }

    fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            let prev = if self.tail == 0 { self.size - 1 } else { self.tail - 1 };
            self.q[prev]
        }
    }

    fn is_empty(&self) -> bool {
        self.q[self.head] == -1
    }

    fn is_full(&self) -> bool {
        self.tail == self.head && self.q[self.head] != -1
    }
}

struct Solution;
