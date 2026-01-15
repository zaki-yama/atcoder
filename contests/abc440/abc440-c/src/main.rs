use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            N: u64,
            W: u64,
            C: [u64; N]
        }
        let mut min = u64::MAX;
        for x in 0..2 * W {
            let mut cost = 0;
            for i in 0..N {
                if (i + x) % (2 * W) < W {
                    cost += C[i as usize];
                }
            }
            if cost < min {
                min = cost;
            }
        }
        println!("{}", min);
    }
}
