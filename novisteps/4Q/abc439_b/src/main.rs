use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    let mut vals = HashSet::new();
    let mut current = n;
    loop {
        vals.insert(current);
        current = current
            .to_string()
            .chars()
            .fold(0, |acc, s| acc + s.to_digit(10).unwrap().pow(2));

        if current == 1 {
            println!("Yes");
            return;
        }

        if vals.contains(&current) {
            println!("No");
            return;
        }
    }
}
