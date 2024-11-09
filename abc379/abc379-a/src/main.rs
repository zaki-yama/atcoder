use proconio::input;

fn main() {
    input! {
        n: u16,
    }
    let a = n / 100;
    let b = (n / 10) % 10;
    let c = n % 10;
    println!("{} {}", b * 100 + c * 10 + a, c * 100 + a * 10 + b);
}
