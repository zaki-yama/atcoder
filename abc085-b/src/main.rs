// -*- coding:utf-8-unix -*-

use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [[i32; 1]; n],
    }

    let remove_duplicate: HashSet<i32> = d.into_iter().flat_map(|x| x).collect();

    println!("{:?}", remove_duplicate.len());
}
