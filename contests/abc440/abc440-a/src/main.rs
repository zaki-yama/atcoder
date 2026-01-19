use proconio::input;

fn main() {
    input! {
        x: u32,
        y: u32,
    }
    println!("{}", x * 2_u32.pow(y));
}
