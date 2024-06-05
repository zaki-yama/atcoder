use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
    }

    for i in 1..b {
        let remainder = i * a % b;
        if remainder == c {
            println!("YES");
            return;
        }
    }
    println!("NO");
}
