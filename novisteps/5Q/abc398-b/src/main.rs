use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        a: [u32; 7],
    }

    let mut map = HashMap::new();
    for &ai in &a {
        *map.entry(ai).or_insert(0) += 1;
    }

    let count_three_or_more = map.values().filter(|&&count| count >= 3).count();
    let count_two_or_more = map.values().filter(|&&count| count >= 2).count();

    if count_three_or_more >= 1 && count_two_or_more >= 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
