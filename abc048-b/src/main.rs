use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        x: u64,
    }
    println!("{}", b / x - a / x);
}
