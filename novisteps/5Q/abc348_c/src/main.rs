use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut beans: [(u32, u32); n],
    }
    let mut min_by_color: HashMap<u32, u32> = HashMap::new();

    beans.iter().for_each(|&(a, c)| {
        let value = min_by_color.get(&c).copied().unwrap_or(u32::MAX);
        if a < value {
            min_by_color.insert(c, a);
        }
    });

    let mut v: Vec<(&u32, &u32)> = min_by_color.iter().collect();
    v.sort_by(|&(_, a), &(_, b)| b.cmp(a));
    println!("{}", v[0].1);
}
