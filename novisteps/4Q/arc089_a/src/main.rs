use proconio::input;

fn main() {
    input! {
        n: usize,
        mut plan: [(i32, i32, i32); n],
    }

    plan.insert(0, (0, 0, 0));
    let yes = plan.windows(2).all(|w| {
        let (t0, x0, y0) = w[0];
        let (t1, x1, y1) = w[1];

        let time = t1 - t0;
        let distance = (x1 - x0).abs() + (y1 - y0).abs();
        distance <= time && (time - distance) % 2 == 0
    });
    println!("{}", if yes { "Yes" } else { "No" });
}
