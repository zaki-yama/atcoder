use proconio::input;

fn main() {
    input! {
        n: usize,
        q_r: [(u32, u32); n],
        q: usize,
        t_d: [(u32, u32); q],
    }
    t_d.iter().for_each(|t_d_i| {
        let t = t_d_i.0;
        let d = t_d_i.1;

        let q = q_r[(t - 1) as usize].0;
        let r = q_r[(t - 1) as usize].1;

        for i in 0..10_u32.pow(9) {
            if d <= q * i + r {
                println!("{}", q * i + r);
                break;
            }
        }
    });
}
