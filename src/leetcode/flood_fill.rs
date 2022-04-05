//! 733. Flood Fill

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let mut image = image;
        Solution::fill_rec(&mut image, sr as usize, sc as usize, new_color);
        image
    }

    fn fill_rec(image: &mut [Vec<i32>], sr: usize, sc: usize, new_color: i32) {
        let current = image[sr][sc];
        
        if new_color == current { return };
        image[sr][sc] = new_color;

        if sr > 0 {
            let left = image[sr-1][sc];
            if left == current {
                Solution::fill_rec(image, sr-1, sc, new_color)
            }
        }

        if sr < image.len() - 1 {
            let right = image[sr+1][sc];
            if right == current {
                Solution::fill_rec(image, sr+1, sc, new_color)
            }
        }

        if sc > 0 {
            let top = image[sr][sc-1];
            if top == current {
                Solution::fill_rec(image, sr, sc-1, new_color)
            }
        }

        if sc < image[sr].len() - 1 {
            let down = image[sr][sc+1];
            if down == current {
                Solution::fill_rec(image, sr, sc+1, new_color)
            }
        }

    }
}
struct Solution;
