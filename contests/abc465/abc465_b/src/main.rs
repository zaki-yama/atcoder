use proconio::input;

fn main() {
    input! {
        x: u32,
        y: u32,
        l: u32,
        r: u32,
        a: u32,
        b: u32,
    }

    if a < l {
        // a < l
        if b < l {
            println!("{}", (b - a) * y);
        } else if b < r {
            // a < l < b < r
            println!("{}", (l - a) * y + (b - l) * x);
        } else {
            // a < l < r < b
            println!("{}", (l - a) * y + (r - l) * x + (b - r) * y);
        }
    } else if a < r {
        // l <= a < r
        if b < r {
            // l < a < b < r
            println!("{}", (b - a) * x);
        } else {
            // l < a < r < b
            println!("{}", (r - a) * x + (b - r) * y);
        }
    } else {
        // r <= a
        println!("{}", (b - a) * y);
    }
}
