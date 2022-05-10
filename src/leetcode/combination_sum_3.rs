///! 216. Combination Sum III


impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut result = vec![vec![]];
        Self::helper(k as usize, n, &mut result, &mut vec![]);
        result
    }

    fn helper(k: usize, n: i32, result: &mut Vec<Vec<i32>>, cur: &mut Vec<i32>) {
        if k == cur.len() && n == 0 {
            result.push(cur.clone());
            return
        }

        let last = *cur.last().unwrap_or(&0) + 1;
        
        for digit in last..=9 {
            if digit <= n {
                cur.push(digit);
                Self::helper(k, n - digit, result, cur);
                cur.pop();
            }
        }

    }
}

struct Solution;
