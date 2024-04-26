// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        (n, a, b): (i32, i32, i32)
    }

    let mut ans = 0;
    for i in 1..=n {
        let sum = i
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .sum::<i32>();
        if a <= sum && sum <= b {
            ans += i;
        }
    }
    println!("{}", ans);
}
