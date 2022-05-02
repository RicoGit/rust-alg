//! 204. Count Primes

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let n = n as usize;
        let mut numbers = vec![true; n + 1];
        let mut counter = 0;
        
        for num in 2..n as usize{
            if numbers[num] {
                for idx in 1..=(n/num) {
                    numbers[idx*num] = false;
                }
                counter += 1;
            }
        }

        counter
    }

    pub fn count_primes_slow(n: i32) -> i32 {
        if n <= 2 {
            return 0
        }

        let mut primes = vec![];

        for num in (3..n as usize).step_by(2) {

            let mut need_add = true;
            for prime in &primes {
                if num * num % prime == 0 {
                    need_add = false;
                    break;
                }
            };
            if need_add {
                primes.push(num);
            }

        }

        primes.len() as i32 + 1
    }
}

struct Solution;


#[cfg(test)]
mod tests {
    use crate::leetcode::count_primes::Solution;

    #[test]
    fn test1() {
        let res = Solution::count_primes(10);
        assert_eq!(res, 4)
    }

    #[test]
    fn test2() {
        let res = Solution::count_primes(3);
        assert_eq!(res, 1)
    }
    #[test]
    fn test3() {
        let res = Solution::count_primes(12);
        assert_eq!(res, 5)
    }

    #[test]
    fn test4() {
        let res = Solution::count_primes(499979);
        assert_eq!(res, 41537)
    }


}
