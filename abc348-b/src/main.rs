// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut coordinates: [(i32, i32); n],  // Vec<(i32, i32, i32)>
    }
    let mut result = vec![];
    // iterate over coordinates
    for point1 in &coordinates {
        let mut farthest = 0.0;
        let mut farthest_index = 0;
        let x1 = point1.0;
        let y1 = point1.1;

        for (index, point2) in coordinates.iter().enumerate() {
            let x2 = point2.0;
            let y2 = point2.1;

            let distance = ((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f64;
            if farthest < distance {
                farthest = distance;
                farthest_index = index + 1;
            }
        }
        result.push(farthest_index);
    }

    for item in result {
        println!("{}", item);
    }
}
