//! 658. Find K Closest Elements

impl Solution {

    // optimal solution
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        let mut start = 0;
        let mut end = arr.len() - k;
        while start < end {
            let mid = start + (end - start) / 2;
            if x - arr[mid] > arr[mid + k] - x {
                start = mid + 1;
            } else {
                end = mid;
            }
        }
        arr[start..(start + k)].into()
    }

    // first solution
    pub fn find_closest_elements_(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {

        fn find_closest(arr: &[i32], start_idx: usize, end_idx: usize, x: i32, k: i32) -> (usize, usize) {
            if end_idx - start_idx == k as usize {
                return (start_idx, end_idx - 1)
            }

            let last_idx = arr.len()-1;

            if start_idx == 0 {
                return (0, k as usize - 1)
            } else if end_idx == last_idx {
                return (last_idx - k as usize, last_idx)
            } else {
                let decr_start_idx = if start_idx == end_idx { start_idx -  1} else { start_idx };
                if (arr[decr_start_idx] - x).abs() <= (arr[end_idx] - x).abs() {
                    find_closest(arr, start_idx - 1, end_idx, x, k)
                } else {
                    find_closest(arr, start_idx, end_idx + 1, x, k)
                }
            }
        }

        if arr.len() == k as usize { return arr };

        let (start_idx, end_idx) = match arr.binary_search(&x) {
            Ok(idx) => {
                find_closest(&arr, idx, idx, x, k)
            },
            Err(idx) => {
                find_closest(&arr, idx, idx, x, k)
            }
        };

        Vec::from(&arr[start_idx..=end_idx])
    }
}



struct Solution;
