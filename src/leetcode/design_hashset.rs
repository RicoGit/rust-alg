//! 705. Design HashSet

struct MyHashSet {
    capacity: usize,
    data: Vec<Vec<i32>>,
}

impl MyHashSet {
    fn new() -> Self {
        let capacity = 500;
        let data = vec![vec![]; capacity];
        MyHashSet { capacity, data }
    }

    fn add(&mut self, key: i32) {
        let idx = self.idx(key);
        if let None = self.data[idx].iter().find(|el| **el == key) {
            self.data[idx].push(key);
        }
    }

    fn remove(&mut self, key: i32) {
        let idx = self.idx(key);
        let mut remove_idx = None;
        for (index, el) in self.data[idx].iter().enumerate() {
            if *el == key {
                remove_idx.replace(index);
                break;
            }
        }
        if let Some(index) = remove_idx {
            self.data[idx].remove(index);
        }
    }

    fn contains(&self, key: i32) -> bool {
        if let Some(keys) = self.data.get(self.idx(key)) {
            return keys.contains(&key);
        }
        false
    }

    fn idx(&self, key: i32) -> usize {
        key as usize % self.capacity
    }
}

struct Solution;
