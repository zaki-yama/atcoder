use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let mut indexes: HashMap<i32, i32> = HashMap::new();
    let mut b: Vec<i32> = vec![];
    for i in 0..n {
        if indexes.contains_key(&a[i]) {
            let val = indexes.get(&a[i]).unwrap();
            b.push(*val);
        } else {
            b.push(-1);
        }
        indexes.insert(a[i], (i + 1) as i32);
    }
    println!(
        "{}",
        b.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
