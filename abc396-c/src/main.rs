use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut b: [i64; n],
        mut w: [i64; m],
    }

    // 降順
    b.sort_by(|a, b| b.cmp(a));
    w.sort_by(|a, b| b.cmp(a));

    // 黒については正数を単純に足し合わせる。取った数も保存しておく
    let (b_count, b_sum) = b
        .iter()
        .filter(|&&x| x > 0)
        .fold((0, 0), |(count, sum), &x| (count + 1, sum + x));

    let mut sum = b_sum;
    // 白についてはまず正数を黒を取った数を超えないところまで加算する
    for i in 0..b_count {
        if w.get(i).is_none() {
            println!("{}", sum);
            return;
        }

        let num = w[i];
        // 負数に到達しても終了
        if num < 0 {
            println!("{}", sum);
            return;
        }
        sum += num;
    }

    // 残った白ボールの処理
    for i in b_count..m {
        // そもそも黒が尽きていたら終了
        if b.get(i).is_none() {
            println!("{}", sum);
            return;
        }

        // 白と黒の合計が負なら取る価値がない
        if b[i] + w[i] < 0 {
            println!("{}", sum);
            return;
        }
        sum += b[i] + w[i];
    }
    println!("{}", sum);
}
