use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [i64; n],
    }

    let mut idx0 = vec![];
    for i in 0..n {
        if a[i] == 0 {
            idx0.push(i);
        }
    }

    if idx0.is_empty() {
        println!("0");
        return;
    }

    let left = s.min(idx0[0]);
    let right = s.max(idx0[idx0.len() - 1] + 1);

    let mut ans: i64 = 0;
    for i in left..right {
        ans += a[i] + 1;
    }
    println!("{}", ans);
}
