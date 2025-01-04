use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut ans = 0;
    for i in 1..=9 {
        for j in 1..=9 {
            if i * j != n {
                ans += i * j;
            }
        }
    }
    println!("{}", ans);
}
