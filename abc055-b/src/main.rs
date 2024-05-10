use proconio::input;

fn main() {
    input! {
        n: u32,
    }
    let mut ans: i64 = 1;
    for i in 1..=n {
        ans *= i as i64;
        ans %= 1_000_000_007;
    }
    println!("{}", ans);
}
