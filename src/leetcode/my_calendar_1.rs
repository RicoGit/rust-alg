//! 729. My Calendar I

use std::collections::BTreeMap;

struct MyCalendar {
    map: BTreeMap<i32, i32>
}

impl MyCalendar {

    fn new() -> Self {
        MyCalendar {
            map: BTreeMap::new()
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        for (_, &prev_end) in self.map.range(..=start).rev().take(1) {
            if prev_end > start { return false }
        }

        for (&next_start, _) in self.map.range(start..).take(1) {
            if next_start < end { return false }
        }

        self.map.insert(start, end);
        true
    }
}