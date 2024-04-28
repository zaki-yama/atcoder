// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],  // Vec<i32>
    }
    a.sort_by(|a, b| b.cmp(a));
    let (a_sum, b_sum) = a
        .iter()
        .enumerate()
        .fold((0, 0), |(a_sum, b_sum), (index, &num)| {
            if index % 2 == 0 {
                (a_sum + num, b_sum)
            } else {
                (a_sum, b_sum + num)
            }
        });
    println!("{}", a_sum - b_sum);
}
