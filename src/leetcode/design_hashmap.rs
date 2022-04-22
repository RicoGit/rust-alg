//! 706. Design HashMap

struct MyHashMap {
    buckets: Vec<Vec<(i32, i32)>>,
}

impl MyHashMap {
    fn new() -> Self {
        MyHashMap {
            buckets: vec![vec![]; 500],
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        self.remove(key);
        let idx = self.idx(key);
        self.buckets[idx].push((key, value));
    }

    fn get(&self, key: i32) -> i32 {
        let idx = self.idx(key);
        if let Some(items) = self.buckets.get(idx) {
            for (k, v) in items {
                if *k == key {
                    return *v;
                }
            }
        }
        -1
    }

    fn remove(&mut self, key: i32) {
        let idx = self.idx(key);
        if let Some(items) = self.buckets.get_mut(idx) {
            let mut remove_idx = None;
            for (i, (k, v)) in items.iter().enumerate() {
                if *k == key {
                    remove_idx = Some(i)
                }
            }
            remove_idx.map(|i| items.remove(i));
        }
    }

    fn idx(&self, key: i32) -> usize {
        key as usize % self.buckets.len()
    }
}

struct Solution;
