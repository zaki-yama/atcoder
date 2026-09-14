use std::cmp::{max, min};

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut coords: [(i32, i32); n],
    }

    let mut minR = i32::MAX;
    let mut maxR = -i32::MAX;
    let mut minC = i32::MAX;
    let mut maxC = -i32::MAX;
    for i in 0..n {
        let (r, c) = coords[i];
        minR = min(minR, r);
        maxR = max(maxR, r);
        minC = min(minC, c);
        maxC = max(maxC, c);
    }
    let ans = (max(maxR - minR, maxC - minC) + 1) / 2;
    println!("{}", ans);
}
