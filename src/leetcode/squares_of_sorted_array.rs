//! 977. Squares of a Sorted Array

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut negative_num_squares = vec![];
        let mut positive_num_squares = vec![];

        // calculate squares of negative numbers
        for (idx, num) in nums.iter().enumerate() {

            // we reach positive numbers
            if *num >= 0 {
                break;
            }

            negative_num_squares.push(num * num);
        }

        // calculate squares of positive numbers
        for (idx, num) in nums.iter().enumerate().skip(negative_num_squares.len()) {
            positive_num_squares.push(num * num);
        }

        // println!("{:?} {:?}", negative_num_squares, positive_num_squares);

        if negative_num_squares.len() == 0 {
            return positive_num_squares
        }

        if positive_num_squares.len() == 0 {
            negative_num_squares.reverse();
            return negative_num_squares
        }

        // join squares of negative and positive numbers
        let mut idx1: i32 = negative_num_squares.len() as i32 - 1;
        let mut idx2: i32 = 0;
        let mut result = Vec::with_capacity(nums.len());

        loop {
            if negative_num_squares[idx1 as usize] < positive_num_squares[idx2 as usize] {
                result.push(negative_num_squares[idx1 as usize]);
                idx1 -= 1;

                if idx1 < 0 {
                    let rest = &positive_num_squares[idx2 as usize..positive_num_squares.len()];
                    result.extend_from_slice(rest);
                    break
                }
            } else {
                result.push(positive_num_squares[idx2 as usize]);
                idx2 += 1;

                if idx2 > positive_num_squares.len() as i32 - 1 {
                    let mut rest = &negative_num_squares[0..idx1 as usize + 1];
                    // println!("rest {:?}", rest);
                    for el in rest.iter().rev() {
                        result.push(*el);
                    }
                    break
                }
            }
        }

        result
    }
}

struct Solution;


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::sorted_squares(vec![0]),
            vec![0]
        );
    }
    #[test]
    fn test3() {
        assert_eq!(
            Solution::sorted_squares(vec![-5,-3,-2,-1]),
            vec![1, 4, 9, 25]
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::sorted_squares(vec![-10000,-9999,-7,-5,0,0,10000]),
            vec![0,0,25,49,99980001,100000000,100000000]
        );
    }
}
