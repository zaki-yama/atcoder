use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        s_and_t: [(u64, u64); n - 1],
    }

    let mut ans = a[0];
    for i in 0..n - 1 {
        let mul = ans / s_and_t[i].0;
        ans = s_and_t[i].1 * mul + a[i + 1];
    }
    println!("{}", ans);
}
