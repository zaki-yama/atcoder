use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut queue = VecDeque::new();
    for i in 0..q {
        input! {
            query_type: usize,
        }
        match query_type {
            1 => {
                input! {
                    c: i64,
                    x: i64,
                }

                queue.push_back((c, x));
            }
            2 => {
                input! {
                    mut k: i64,
                }

                let mut sum = 0;
                while !queue.is_empty() && queue[0].0 <= k {
                    sum += queue[0].0 * queue[0].1;
                    k -= queue[0].0;
                    queue.pop_front();
                }
                if k != 0 {
                    sum += k * queue[0].1;
                    queue[0] = (queue[0].0 - k, queue[0].1);
                }
                println!("{}", sum);
            }
            _ => panic!(),
        }
    }
}
