use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
    }

    let mut remainders: HashSet<u32> = HashSet::new();
    let mut i = 1;
    loop {
        let remainder = i * a % b;
        if remainder == c {
            println!("YES");
            return;
        }

        if remainders.contains(&remainder) {
            println!("NO");
            return;
        } else {
            remainders.insert(remainder);
            i += 1;
        }
    }
}
