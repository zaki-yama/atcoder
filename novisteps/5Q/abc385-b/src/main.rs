use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        x: usize,
        y: usize,
        s: [Chars; h],
        t: Chars,
    }

    let mut current = (x - 1, y - 1);
    let mut houses: HashSet<(usize, usize)> = HashSet::new();

    for i in 0..t.len() {
        let next = match t[i] {
            'U' => (current.0 - 1, current.1),
            'D' => (current.0 + 1, current.1),
            'L' => (current.0, current.1 - 1),
            'R' => (current.0, current.1 + 1),
            _ => panic!("Never"),
        };
        match s[next.0][next.1] {
            '#' => {}
            '.' => {
                current = next;
            }
            '@' => {
                current = next;
                houses.insert(next);
            }
            _ => {}
        }
    }
    println!("{} {} {}", current.0 + 1, current.1 + 1, houses.len());
}
