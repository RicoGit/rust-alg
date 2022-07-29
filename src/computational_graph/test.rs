
use std::ops::Add;

struct Fib<T: Add + Add<Output = T> + Copy> {
    a: T,
    b: T
}

impl <T: Add + Add<Output = T> + Copy> Iterator for Fib<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.a;
        let tmp = self.a + self.b;
        self.a = self.b;
        self.b = tmp;
        Some(res)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut iter = Fib {
            a: 1f32,
            b: 1f32
        };

        for idx in 0..9 {
            println!("{:?}", iter.next())
        }

    }

}