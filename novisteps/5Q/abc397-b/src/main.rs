use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 0;
    for window in s.windows(2) {
        if window[0] == window[1] {
            ans += 1;
        }
    }

    if s[0] == 'o' {
        ans += 1;
    }
    if s[s.len() - 1] == 'i' {
        ans += 1;
    }
    println!("{}", ans);
}
