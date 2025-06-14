use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
        k: u32,
    }
    let mut ans = 0;
    for i in 0..n {
        if k <= a[i] {
            ans += 1;
        }
    }
    println!("{}", ans);
}
