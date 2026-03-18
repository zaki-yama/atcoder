use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut ans = vec![];
    let mut cursor = 0;
    for i in 0..t.len() {
        if t[i] == s[cursor] {
            ans.push(i + 1);
            cursor += 1;
        }
    }
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
}
