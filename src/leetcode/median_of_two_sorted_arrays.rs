//! 4. Median of Two Sorted Arrays
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total = nums1.len() + nums2.len();

        // easy way to compute for small input arrays
        if total < 8 || nums1.is_empty() || nums2.is_empty() {
            return Self::direct_find_median_sorted_arrays(nums1, nums2);
        }

        let half = total / 2;
        let (small, big) = if nums1.len() < nums2.len() {
            (nums1, nums2)
        } else {
            (nums2, nums1)
        };
        println!("t:{} h:{} | small:{:?} big:{:?}", total, half, small, big);

        let mut small_half = &small[..small.len() / 2]; // half of smallest array
        let mut big_half = &big[0..0]; // empty as start

        while small_half.len() + big_half.len() != half {
            let last = small_half.last().unwrap_or(&i32::MIN);

            let mut search_idx = 0;
            match big.binary_search(&last) {
                Ok(idx) => search_idx = idx,
                Err(idx) => search_idx = idx,
            }

            if search_idx == 0 {
                small_half = &small[0..small_half.len() + 1];
                if big_half.len() > 0 {
                    big_half = &big[0..big_half.len() - 1];
                }
            } else if search_idx >= big_half.len() {
                if small_half.len() > 0 {
                    small_half = &small[0..small_half.len() - 1];
                }
                big_half = &big[0..big_half.len() + 1];
            } else {
                big_half = &big[0..search_idx]
            }
            println!("small_half:{:?} big_half:{:?}", small_half, big_half);
        }

        if total % 2 == 0 {
            let first = *small_half
                .last()
                .unwrap_or(&i32::MIN)
                .max(big_half.last().unwrap()) as f64;
            let second = *small
                .get(small_half.len())
                .unwrap_or(&i32::MAX)
                .min(big.get(big_half.len()).unwrap_or(&i32::MAX)) as f64;
            return (first + second) / 2.0;
        } else {
            return *small
                .get(small_half.len())
                .unwrap_or(&i32::MIN)
                .max(big.get(big_half.len()).unwrap()) as f64;
        }
    }
    // dirty hack, can't be considered as solution
    pub fn direct_find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums: Vec<i32> = vec![];
        if nums1.is_empty() {
            nums = nums2
        } else if nums2.is_empty() {
            nums = nums1
        } else {
            nums.extend_from_slice(&nums1);
            nums.extend_from_slice(&nums2);
            nums.sort_unstable();
        }

        let len = nums.len();
        if len % 2 == 1 {
            return nums[len / 2] as f64;
        } else {
            return (nums[len / 2] + nums[len / 2 - 1]) as f64 / 2.0;
        }
    }
}
struct Solution;
