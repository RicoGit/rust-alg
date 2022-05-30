//! 29. Divide Two Integers

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let negative = (dividend < 0) != (divisor < 0);

        let mut divisor = (divisor as i64).abs();
        let mut dividend = (dividend as i64).abs();

        if divisor == 1 { return compose(dividend, negative) }
        if dividend < divisor { return 0 }

        let mut n = 1;
        while dividend > divisor {
            divisor <<= 1;
            n <<= 1;
        }

        let mut result = 0;
        while n > 0 {
            if dividend >= divisor {
                result += n;
                dividend -= divisor;
            }
            divisor >>= 1;
            n >>= 1;
        }

        compose(result, negative)
    }
}

fn compose(result: i64, negative: bool) -> i32 {
    (if negative {
        -result
    } else {
        result.min(i64::from(std::i32::MAX))
    }) as i32
}

struct Solution;
