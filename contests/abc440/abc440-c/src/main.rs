use proconio::input;
use std::collections::BTreeMap;

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

        // 初期コスト (x=0) を計算
        let mut cost = 0;
        for i in 0..N {
            if i % (2 * W) < W {
                cost += C[i as usize];
            }
        }
        let mut min = cost;

        // イベントを記録: 各 x でのコスト変化量
        let mut events: BTreeMap<u64, i64> = BTreeMap::new();

        for i in 0..N {
            let pos = i % (2 * W);

            if pos < W {
                // x=0 で選択されている → (i+x) % (2*W) = W で外れる
                let x = (W - pos) % (2 * W);
                if x > 0 {
                    *events.entry(x).or_insert(0) -= C[i as usize] as i64;
                }
            } else {
                // x=0 で選択されていない → (i+x) % (2*W) = 0 で入る
                let x = (2 * W - pos) % (2 * W);
                if x > 0 {
                    *events.entry(x).or_insert(0) += C[i as usize] as i64;
                }
            }
        }

        // イベントを順番に適用して最小値を更新
        for (_x, &delta) in events.iter() {
            cost = ((cost as i64) + delta) as u64;
            if cost < min {
                min = cost;
            }
        }

        println!("{}", min);
    }
}
