//! 341. Flatten Nested List Iterator

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
  Int(i32),
  List(Vec<NestedInteger>)
}

struct NestedIterator {
    pub vec: Vec<i32>
}

use NestedInteger::{Int, List};

impl NestedIterator {

    fn new(mut nestedList: Vec<NestedInteger>) -> Self {
        let mut vec = vec![];
        Self::init(&mut nestedList, &mut vec);
        NestedIterator {
            vec
        }
    }

    fn init(nestedList: &mut Vec<NestedInteger>, vec: &mut Vec<i32>) {
        while let Some(nested) = nestedList.pop() {
            match nested {
                Int(val) => {
                    vec.push(val)
                },
                List(mut values) => {
                    nestedList.append(&mut values);
                    Self::init(nestedList, vec);
                }
            }
        }

    }

    fn next(&mut self) -> i32 {
        self.vec.pop().unwrap()
    }

    fn has_next(&self) -> bool {
        !self.vec.is_empty()
    }
}

struct Solution;
