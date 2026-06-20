use proconio::input;

fn main() {
    input! {
        x: u32,
        y: u32,
    }

    if 9 * x == 16 * y {
        println!("Yes");
    } else {
        println!("No");
    }
}
