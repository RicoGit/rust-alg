//! 1631. Path With Minimum Effort

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let rows = heights.len();
        let colls = heights[0].len();

        let mut efforts = vec![vec![i32::MAX; colls]; rows];
        efforts[0][0] = 0;

        let mut stack = vec![(0, 0)];

        while let Some((r, c)) = stack.pop() {
            let cur = heights[r][c];
            let cur_eff = efforts[r][c];

            // check right
            if c < colls-1 {
                Self::process(r, c, r, c+1, &heights, &mut efforts, &mut stack);
            }
            // check left
            if c > 0 {
                Self::process(r, c, r, c-1, &heights, &mut efforts, &mut stack);
            }
            // check top
            if r > 0 {
                Self::process(r, c, r-1, c, &heights, &mut efforts, &mut stack);
            }
            // check down
            if r < rows-1 {
                Self::process(r, c, r+1, c, &heights, &mut efforts, &mut stack);
            }
        }

        // println!("{:?}", efforts);
        efforts[rows-1][colls-1]
    }

    fn process(
        cur_r: usize,
        cur_c: usize,
        r: usize,
        c: usize,
        heights: &Vec<Vec<i32>>,
        efforts: &mut Vec<Vec<i32>>,
        stack: &mut Vec<(usize, usize)>
    ) {
        let cur = heights[cur_r][cur_c];
        let next = heights[r][c];
        let next_eff = (next-cur).abs();

        let cur_eff = efforts[cur_r][cur_c];
        let old_next_eff = efforts[r][c];
        if next_eff.max(cur_eff) < old_next_eff {
            efforts[r][c] = next_eff.max(cur_eff);
            stack.push((r, c));
        }
    }
}

struct Solution;
