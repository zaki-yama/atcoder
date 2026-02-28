use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut m: HashMap<char, u32> = HashMap::new();
    s.iter().for_each(|&ch| {
        m.insert(ch, m.get(&ch).copied().unwrap_or(0) + 1);
    });

    let mut v: Vec<(&char, &u32)> = m.iter().collect();
    v.sort_by_key(|&(_, val)| val);

    let mut current = 0;
    for chunk in v.chunks(2) {
        if *chunk[0].1 == current {
            println!("No");
            return;
        }
        if !(chunk.len() == 2 && chunk[0].1 == chunk[1].1) {
            println!("No");
            return;
        }
        current = *chunk[0].1;
    }
    println!("Yes");
}
