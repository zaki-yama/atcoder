use proconio::input;

fn main() {
    input! {
        n: usize,
        points: [(i64, i64); n],
    }
    let mut ans: f64 = 0.0;
    // 原点と最初の点の距離を足す
    let distance = ((0 - points[0].0).pow(2) + (0 - points[0].1).pow(2)) as f64;
    ans += distance.sqrt();

    for i in 0..n - 1 {
        let distance = ((points[i].0 - points[i + 1].0).pow(2)
            + (points[i].1 - points[i + 1].1).pow(2)) as f64;
        ans += distance.sqrt();
    }
    // ここで最後の点と原点の距離を足す
    let distance = ((points[n - 1].0 - 0).pow(2) + (points[n - 1].1 - 0).pow(2)) as f64;
    ans += distance.sqrt();
    println!("{}", ans);
}
