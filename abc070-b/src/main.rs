use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
    }
    println!("{}", (b.min(d) - a.max(c)).max(0));
}
