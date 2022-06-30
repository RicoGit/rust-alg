//! 1491. Average Salary Excluding the Minimum and Maximum Salary

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let mut max = i32::MIN;
        let mut min = i32::MAX;
        let mut sum = 0;
        let len = salary.len();

        for s in salary {
            sum += s;
            max = max.max(s);
            min = min.min(s);
        }

        (sum - max - min) as f64 / (len - 2) as f64
    }
}

struct Solution;