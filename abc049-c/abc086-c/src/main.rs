// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut points: [(i32, i32, i32); n],  // Vec<(i32, i32, i32)>
    }

    for i in 0..points.len() {
        let t_delta = if i == 0 {
            points[i].0
        } else {
            (points[i].0 - points[i - 1].0).abs()
        };
        let x_delta = if i == 0 {
            points[i].1
        } else {
            (points[i].1 - points[i - 1].1).abs()
        };
        let y_delta = if i == 0 {
            points[i].2
        } else {
            (points[i].2 - points[i - 1].2).abs()
        };
        if t_delta < x_delta + y_delta || (t_delta - (x_delta + y_delta)) % 2 != 0 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
