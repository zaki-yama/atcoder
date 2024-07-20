use proconio::input;

fn main() {
    input! {
        r: u32,
    }
    let ans = match r {
        1..=99 => 100 - r,
        100..=199 => 200 - r,
        200..=299 => 300 - r,
        _ => panic!(),
    };
    println!("{}", ans);
}
