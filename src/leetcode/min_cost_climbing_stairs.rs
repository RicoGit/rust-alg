//! 746. Min Cost Climbing Stairs

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let MAX = 1000;
        let mut buf = vec![0; cost.len()];

        for idx in (0..cost.len()).rev() {
            let one_step = buf.get(idx+1).unwrap_or(&0);
            let two_steps = buf.get(idx+2).unwrap_or(&0);
            buf[idx] = cost[idx] + one_step.min(two_steps);
        }

        println!("{:?}", buf);
        buf[0].min(buf[1])
    }
}

struct Solution;
