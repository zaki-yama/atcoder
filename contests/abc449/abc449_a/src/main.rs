use std::f64::consts::PI;

use proconio::input;

fn main() {
    input! {
        d: f64,
    }
    println!("{}", PI * ((d / 2.0).powf(2.0) as f64));
}
