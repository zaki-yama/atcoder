use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
        l_r: [(usize, usize); m],
    }

    let mut v = vec![0; n];
    for i in 0..m {
        let (l, r) = l_r[i];
        v[l - 1] += 1;
        if r < n {
            v[r] -= 1;
        }
    }

    let mut ans = vec![];
    let mut current = 0;
    for i in 0..n {
        current += v[i];
        if current % 2 == 0 {
            ans.push(s[i]);
        } else {
            ans.push(t[i]);
        }
    }
    println!("{}", ans.into_iter().collect::<String>());
}
