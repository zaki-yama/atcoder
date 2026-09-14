use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for i in 0..t {
        input! {
            n: usize,
            d: usize,
            a: [u32; n],
            b: [u32; n],
        }

        let mut queue = VecDeque::new();

        for i in 0..n {
            for _j in 0..a[i] {
                queue.push_back(i);
            }
            for _j in 0..b[i] {
                queue.pop_front();
            }
            while !queue.is_empty() && queue.front().is_some_and(|&v| v + d <= i) {
                queue.pop_front();
            }
        }
        println!("{}", queue.len());
    }
}
