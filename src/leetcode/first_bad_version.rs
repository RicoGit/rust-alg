//! 278. First Bad Version

struct Solution();

impl Solution {


    // The API isBadVersion is defined for you.
    // to call it use self.isBadVersion(version)
    fn isBadVersion(&self, _n: i32) -> bool {
        todo!();
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
        self.search_rec(0, n as usize)
    }


    fn search_rec(&self, start: usize, end: usize) -> i32 {
        if end == start {
            if self.isBadVersion(end as i32) {
                return start as i32;
            } else {
                return -1;
            }
        }

        let mid = dbg!((start / 2) + (end / 2));

        if self.isBadVersion(mid as i32) {
            self.search_rec(start, mid)
        } else {
            self.search_rec(mid + 1, end)
        }
    }
}
