use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    for f in 0..t {
        input! {
            N: usize,
            W: usize,
            C: [u64; N]
        }

        // 長さ2Wのコスト配列を作る
        let mut sum_costs = vec![0; 2 * W];

        for i in 0..N {
            sum_costs[i % (2 * W)] += C.get(i).copied().unwrap_or(0);
        }

        // 初期値は 0..W
        let mut current: u64 = sum_costs[0..W].iter().sum();
        let mut ans = current;

        for i in 1..2 * W {
            // i 番目は、current + C[i+W-1] - C[i-1]
            current += sum_costs[(i + W - 1) % (2 * W)];
            current -= sum_costs[i - 1];
            if ans > current {
                ans = current;
            }
        }

        println!("{}", ans);
    }
}
