//! 155. Min Stack

struct MinStack {
    vec: Vec<i32>
}

impl MinStack {

    fn new() -> Self {
        MinStack {
            vec: vec![]
        }
    }

    fn push(&mut self, val: i32) {
        self.vec.push(val)
    }

    fn pop(&mut self) {
        self.vec.pop();
    }

    fn top(&self) -> i32 {
        *self.vec.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.vec.iter().min().unwrap()
    }
}
