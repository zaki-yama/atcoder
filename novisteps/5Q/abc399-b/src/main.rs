use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [u32; n]
    }
    let mut score_rank_map = HashMap::new();

    let mut sorted = p.clone();
    sorted.sort_by(|a, b| b.cmp(a));
    for i in 0..n {
        score_rank_map.entry(sorted[i]).or_insert(i + 1);
    }
    p.iter().for_each(|pi| {
        println!("{}", score_rank_map[pi]);
    });
}
