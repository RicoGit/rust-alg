//! 630. Course Schedule III

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn schedule_course(mut courses: Vec<Vec<i32>>) -> i32 {
        courses.sort_by_key(|course| course[1]);
        let mut durations = BinaryHeap::new();

        let mut time = 0;

        for idx in 0..courses.len() {
            let dur = courses[idx][0];
            let end = courses[idx][1];

            if time + dur <= end {
                time += dur;
                durations.push(dur)
            } else {
                if let Some(max_duration) = durations.pop() {
                    if max_duration > dur {
                        time = time - max_duration + dur; // recalculate time
                        durations.push(dur); // smaller duration is better
                    } else {
                        durations.push(max_duration); // return old value back
                    }
                }
            }
        }
        durations.len() as i32
    }

    // more concise version
    pub fn schedule_course_easy(mut courses: Vec<Vec<i32>>) -> i32 {
        courses.sort_by_key(|course| course[1]);
        let mut durations = BinaryHeap::new();
        let mut time = 0;

        for course in courses {
            time += course[0];
            durations.push(course[0]);
            if time > course[1] {
                if let Some(max_dur) = durations.pop() {
                    time -= max_dur;
                }
            }

        }
        durations.len() as i32
    }
}

struct Solution;