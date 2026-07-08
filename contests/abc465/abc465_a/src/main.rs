use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    }
    println!("{}", if a > b * 2 / 3 { "Yes" } else { "No" });
}
