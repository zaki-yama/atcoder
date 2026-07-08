use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut c = 0;
    let mut ans = VecDeque::new();
    for i in 1..=n {
        if c % 2 == 0 {
            ans.push_back(i);
        } else {
            ans.push_front(i);
        }

        if s[i - 1] == 'o' {
            c += 1;
        }
    }
    if c % 2 == 1 {
        ans.make_contiguous().reverse();
    }

    println!(
        "{}",
        ans.iter()
            .map(|x| { x.to_string() })
            .collect::<Vec<_>>()
            .join(" ")
    );
}
