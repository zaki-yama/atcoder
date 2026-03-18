use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
        t: [Chars; m],
    }
    for i in 0..=n - m {
        for j in 0..=n - m {
            if matches(&s, &t, i, j) {
                println!("{} {}", i + 1, j + 1);
                return;
            }
        }
    }
}

fn matches(s: &Vec<Vec<char>>, t: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    for p in 0..t.len() {
        for q in 0..t[p].len() {
            if s[p + i][q + j] != t[p][q] {
                return false;
            }
        }
    }
    return true;
}
