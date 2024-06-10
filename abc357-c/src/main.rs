use proconio::input;

fn main() {
    input! {
        N: u32,
    }

    let mut carpets: Vec<Vec<Vec<char>>> = vec![];
    carpets.push(vec![vec!['#']]);

    // 初期化
    for i in 1..=N {
        let size = 3_usize.pow(i as u32);
        let array = vec![vec!['.'; size]; size];
        carpets.push(array);
    }

    for i in 1..=N {
        for j in 0..3_i32.pow(i) {
            for k in 0..3_i32.pow(i) {
                if 3_i32.pow(i - 1) <= j
                    && j < 2 * 3_i32.pow(i - 1)
                    && 3_i32.pow(i - 1) <= k
                    && k < 2 * 3_i32.pow(i - 1)
                {
                    // 中央なので何もしない
                } else {
                    // 1つ下のレベルのcarpetから取ってくる
                    // 座標(j, k)を 3^i-1 で割った余りの座標でレベルi-1のcarpetを参照する
                    let j2 = j % 3_i32.pow(i - 1);
                    let k2 = k % 3_i32.pow(i - 1);
                    carpets[i as usize][j as usize][k as usize] =
                        carpets[(i - 1) as usize][j2 as usize][k2 as usize];
                }
            }
        }
    }

    let target = carpets.last().unwrap();
    for row in target {
        let s: String = row.iter().collect();
        println!("{}", s);
    }
}
