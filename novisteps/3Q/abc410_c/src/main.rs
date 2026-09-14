use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut a = Vec::with_capacity(n);
    for i in 0..n {
        a.push(i + 1);
    }
    let mut cursor = 0;
    for _ in 0..q {
        input! {
            q_t: usize,
        }

        match q_t {
            1 => {
                input! {
                    p: usize,
                    x: usize,
                }
                // println!("query 1: {} {}", p, x);
                let idx = (p - 1 + cursor) % n;
                a[idx] = x;
            }
            2 => {
                input! {
                    p: usize,
                }
                // println!("query 2: {}", p);
                let idx = (p - 1 + cursor) % n;
                println!("{}", a[idx]);
            }
            3 => {
                input! {
                    k: usize,
                }
                // println!("query 3: {}", k);
                cursor += k;
                cursor %= n;
            }
            _ => panic!(),
        }
    }
}
