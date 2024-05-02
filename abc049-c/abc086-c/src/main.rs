// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut points: [(i32, i32, i32); n],  // Vec<(i32, i32, i32)>
    }

    let mut t_prev = 0;
    let mut x_prev = 0;
    let mut y_prev = 0;

    for &(t, x, y) in &points {
        let t_delta = (t - t_prev).abs();
        let x_delta = (x - x_prev).abs();
        let y_delta = (y - y_prev).abs();

        let dist = x_delta + y_delta;

        if t_delta < dist || t_delta % 2 != dist % 2 {
            println!("No");
            return;
        }

        t_prev = t;
        x_prev = x;
        y_prev = y;
    }
    println!("Yes");
}
