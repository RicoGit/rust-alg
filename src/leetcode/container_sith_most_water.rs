//! 11. Container With Most Water

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {

        let mut max = usize::MIN;
        let mut start = 0;
        let mut end = height.len()-1;

        loop {
            let current = Solution::square(&height, start, end);

            max = max.max(current);

            if height[start] > height[end] {
                end -= 1;
            } else {
                start += 1;
            }

            if start >= end {
                break
            }
        }

        max as i32
    }

    fn square(height: &[i32], start: usize, end: usize) -> usize {
        let height = height[start].min(height[end]) as usize; // ok, height is always non-negative
        let width = end - start;

        width * height
    }
}

struct Solution;
