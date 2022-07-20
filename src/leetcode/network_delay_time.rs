//! 743. Network Delay Time

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = (k - 1) as usize; // make addressing from 0

        let mut distances = vec![i32::MAX; n];
        distances[k] = 0;

        // build graph
        let mut graph: Vec<Vec<(usize, i32)>> = vec![vec![]; n];
        for time in times {
            // make addressing from 0
            let src = (time[0] - 1) as usize;
            let trg = (time[1] - 1) as usize;
            let dist = time[2];
            graph[src].push((trg, dist))
        }

        for mut node in &mut graph {
            node.sort_by_key(|el| el.1)
        }

        let mut stack = vec![];
        stack.push(k);
        while let Some(idx) = stack.pop() {
            for (target, dist) in &graph[idx] {
                let old_dist = distances[*target];
                let new_dist = dist + distances[idx];
                if old_dist > new_dist {
                    // recalculate target
                    distances[*target] = new_dist;
                    stack.push(*target);
                }
            }
        }

        // println!("{:?} >> {:?}", graph, distances);
        let max = *distances.iter().max().unwrap();
        if max == i32::MAX {
            -1
        } else {
            max
        }
    }
}

struct Solution;
