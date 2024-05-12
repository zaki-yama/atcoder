use proconio::input;

fn main() {
    input! {
        n: u32,
        k: u32,
    }
    println!("{}", k * (k - 1).pow(n - 1));
}
