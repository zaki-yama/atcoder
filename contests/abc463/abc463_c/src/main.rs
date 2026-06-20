use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        h_l: [(u32, u32); n],  // Vec<(i32, i32)>
        q: usize,
        t: [u32; q],
    }
    let mut sorted_h_l = h_l.clone();
    sorted_h_l.sort_by(|a, b| b.0.cmp(&a.0));

    let mut l_h = VecDeque::new();
    let mut current_l = 0;
    for i in 0..n {
        if current_l < sorted_h_l[i].1 {
            l_h.push_back((sorted_h_l[i].1, sorted_h_l[i].0));
            current_l = sorted_h_l[i].1;
        }
    }

    for i in 0..q {
        for j in 0..l_h.len() {
            if t[i] < l_h[j].0 {
                println!("{}", l_h[j].1);
                break;
            }
        }
    }
}
