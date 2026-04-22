use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
    }

    // 頻度を集計
    let mut cnt = vec![0i64; n];

    for i in 0..n {
        cnt[(a[i] - 1) as usize] += 1;
    }

    let mut ans = 0;
    for i in 0..n {
        ans += cnt[i] * (cnt[i] - 1) * (n as i64 - cnt[i]) / 2;
    }
    println!("{}", ans);
}
