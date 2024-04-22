// -*- coding:utf-8-unix -*-

use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut result: HashSet<&[char]> = HashSet::new();
    for i in 0..s.len() {
        for j in i..s.len() {
            let sub = &s[i..=j];
            result.insert(sub);
        }
    }
    println!("{}", result.len());
}
