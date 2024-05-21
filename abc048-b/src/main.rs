use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        x: u64,
    }
    let bx = b / x;
    let ax = a / x;
    let ans = if a % x == 0 { bx - ax + 1 } else { bx - ax };

    println!("{}", ans);
}
