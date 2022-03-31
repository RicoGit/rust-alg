//! John works at a clothing store. He has a large pile of socks that he must pair by color for sale.
//! Given an array of integers representing the color of each sock, determine how many pairs of socks
//! with matching colors there are.
//!
//! For example, there are
//! socks with colors . There is one pair of color and one of color . There are three odd socks left,
//! one of each color. The number of pairs is .
//!
//! [hackerrank](https://www.hackerrank.com/challenges/sock-merchant/problem)

use std::collections::HashSet;
use std::io;
use std::str::FromStr;

pub fn run() {
    let _n = i32::from_str(&read_str().trim()).unwrap(); // just ignore it

    let arr: Vec<i32> = read_str()
        .trim()
        .split(" ")
        .map(|el| i32::from_str(el.trim()).unwrap())
        .collect();

    let result = sock_merchant(&arr);

    println!("{:?}", result);
}

// Complete the sockMerchant function below.
fn sock_merchant(arr: &[i32]) -> i32 {
    struct Acc {
        res: i32,
        set: HashSet<i32>,
    }
    let res = 0;
    let set = HashSet::<i32>::new();

    arr.iter()
        .fold(Acc { res, set }, |mut acc, el| {
            if acc.set.remove(el) {
                acc.res = acc.res + 1;
            } else {
                acc.set.insert(*el);
            }
            acc
        })
        .res
}

fn read_str() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer
}
