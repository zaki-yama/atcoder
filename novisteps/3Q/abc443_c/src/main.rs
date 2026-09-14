use proconio::input;

fn main() {
    input! {
        n: usize,
        t: u32,
        a: [u32; n],
    }

    let mut ans = 0;
    let mut next = 0;
    for i in 0..n {
        if a[i] > next {
            ans += a[i] - next;
            next = a[i] + 100;
        }
    }
    if t > next {
        ans += t - next;
    }
    println!("{}", ans);
}
