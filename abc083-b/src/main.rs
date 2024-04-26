// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        (n, a, b): (i32, i32, i32)
    }

    let ans = (1..=n)
        .filter(|&i| {
            let sum = sum_of_digits(i);
            a <= sum && sum <= b
        })
        .sum::<i32>();

    println!("{}", ans);
}

fn sum_of_digits(mut num: i32) -> i32 {
    let mut sum = 0;
    while num > 0 {
        sum += num % 10;
        num /= 10;
    }
    sum
}
