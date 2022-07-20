//! 1710. Maximum Units on a Truck

impl Solution {
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, mut truck_size: i32) -> i32 {
        box_types.sort_unstable_by_key(|vec| vec[1]);
        let mut result = 0;
        for b in box_types.into_iter().rev() {
            truck_size -= b[0];
            result += b[0] * b[1];
            while truck_size < 0 {
                result -= b[1];
                truck_size += 1;
            }
        }
        result
    }
}

struct Solution;
