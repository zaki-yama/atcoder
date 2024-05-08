use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
    }
    if d < a || b < c {
        println!("0");
        return;
    }

    println!("{}", b.min(d) - a.max(c));
}
