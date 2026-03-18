use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        s: String,
    }
    let r = r + 1;
    let s: Vec<usize> = s.bytes().map(|b| (b - b'a') as usize).collect();
    let mut ans: i64 = 0;
    let mut cnt = vec![0i64; 26];
    for i in 0..n {
        if i >= l {
            cnt[s[i - l]] += 1;
        }
        if i >= r {
            cnt[s[i - r]] -= 1;
        }
        ans += cnt[s[i]];
    }
    println!("{}", ans);
}
