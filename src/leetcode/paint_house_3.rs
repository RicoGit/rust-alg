//! 1473. Paint House III

impl Solution {
    pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        let MAX_COST = 10000001;
        let mut buf = vec![vec![vec![MAX_COST; n as usize]; target as usize + 1]; m as usize];

        // Initialize for house 0, neighborhoods will be 1
        for color in 1..=n {
            if houses[0] == color {
                // If the house has same color, no cost
                buf[0][1][color as usize - 1] = 0;
            } else if houses[0] == 0 {
                // If the house is not painted, assign the corresponding cost
                buf[0][1][color as usize -1] = cost[0][color as usize - 1]
            }
        }

        for h_idx in 1..houses.len() {
            for n_idx in 1..=(h_idx + 1).min(target as usize) {
                for color in 1..=n {
                    let c_idx = color as usize;
                    let house = houses[h_idx];

                    // If the house is already painted, and color is different
                    if house != 0 && color != house { continue }

                    let mut current_cost = MAX_COST;
                    // Iterate over all the possible color for previous house
                    for prev_color in 1..=n {
                        if color != prev_color {
                            // Decrement the neighborhood as adjacent houses has different color
                            current_cost = current_cost.min(buf[h_idx - 1][n_idx-1][prev_color as usize - 1]);
                        } else {
                            // Previous house has the same color, no change in neighborhood count
                            current_cost = current_cost.min(buf[h_idx - 1][n_idx][c_idx - 1]);
                        }
                    }

                    // If the house is already painted, cost to paint is 0
                    let cost_to_paint = if house != 0 { 0 } else { cost[h_idx][c_idx - 1] };
                    buf[h_idx][n_idx][c_idx - 1] = current_cost + cost_to_paint;
                }
            }
        }
        println!("{:?}", buf);
        let res = *buf[m as usize - 1][target as usize].iter().min().unwrap_or(&-1);
        if res >= MAX_COST { -1 } else { res }
    }
}

struct Solution;