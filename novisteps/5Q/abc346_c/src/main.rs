use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64,
        mut a: [u64; n],
    }

    let values: HashSet<u64> = a.into_iter().collect();
    let mut ans = k * (k + 1) / 2;
    for value in values {
        if value <= k {
            ans -= value;
        }
    }
    println!("{:?}", ans);
}
