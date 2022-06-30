//! 406. Queue Reconstruction by Height

impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = Vec::with_capacity(people.len());

        people.sort_unstable_by(|a, b| {
            a[0].cmp(&b[0]).then(b[1].cmp(&a[1]))
        });

        for p in people.iter().rev() {
            result.insert(p[1] as usize, p.to_vec())
        }

        result
    }
}


struct Solution;