// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],  // Vec<i32>
    }
    let mut result = 0;
    loop {
        let is_all_even = a.iter().all(|x| x % 2 == 0);
        if is_all_even {
            result += 1;
            // If all items are even, divide all items by 2
            a = a.iter().map(|x| x / 2).collect();
        } else {
            break;
        }
    }

    println!("{}", result);
}
