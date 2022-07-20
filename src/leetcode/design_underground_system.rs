//! 1396. Design Underground System

use std::collections::HashMap;

struct UndergroundSystem {
    on_the_way: HashMap<i32, (String, i32)>,
    avg_time: HashMap<(String, String), (i32, usize)>,
}

impl UndergroundSystem {
    fn new() -> Self {
        UndergroundSystem {
            on_the_way: HashMap::new(),
            avg_time: HashMap::new(),
        }
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.on_the_way.insert(id, (station_name, t));
    }

    fn check_out(&mut self, id: i32, end_st: String, t: i32) {
        if let Some((start_st, start_t)) = self.on_the_way.remove(&id) {
            let dur = t - start_t;
            let key = (start_st, end_st);

            match self.avg_time.remove(&key) {
                None => self.avg_time.insert(key, (dur, 1)),
                Some((old_sum, old_count)) => {
                    self.avg_time.insert(key, (old_sum + dur, old_count + 1))
                }
            };
        }
    }

    fn get_average_time(&self, start_st: String, end_st: String) -> f64 {
        if let Some((sum, count)) = self.avg_time.get(&(start_st, end_st)) {
            return *sum as f64 / *count as f64;
        }
        0.0
    }
}

struct Solution;
