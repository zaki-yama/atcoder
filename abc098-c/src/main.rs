use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut chars: Chars,
    }

    let mut s = vec![];
    s.push(0);

    let mut count = 0;
    for &ch in &chars {
        if ch == 'W' {
            count += 1;
        }
        s.push(count);
    }


    for (i, val) in s.iter().enumerate() {
        println!("s[{}] = {}", i, val);
    }
}
