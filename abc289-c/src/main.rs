use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut s = Vec::new();
    for _i in 0..m {
        input! {
            c: usize,
            mut a: [u32; c],
        }
        s.push(a);
    }

    let mut ans = 0;
    for bit in 0..(1 << m) {
        let mut included: HashSet<u32> = HashSet::new();
        for i in 0..m {
            if bit & (1 << i) != 0 {
                included.extend(s[i].iter().cloned().collect::<HashSet<u32>>());
            }
        }

        if contains_all(&included, n as u32) {
            ans += 1;
        }
    }
    println!("{}", ans);
}

fn contains_all(set: &HashSet<u32>, n: u32) -> bool {
    (1..=n).all(|v| set.contains(&v))
}
