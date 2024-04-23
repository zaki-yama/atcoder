// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        n: usize,
    }
    let result = (1..=n)
        .map(|i| if i % 3 == 0 { 'x' } else { 'o' })
        .collect::<String>();
    println!("{}", result);
}
