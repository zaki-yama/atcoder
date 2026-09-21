use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    if n < k {
        println!("1");
        return;
    }

    let mut a: Vec<u128> = vec![1; n + 1];
    a[k] = k as u128;
    for i in k + 1..=n {
        a[i] = 10_u128.pow(9) + 2 * a[i - 1] - a[i - k - 1];
        a[i] = a[i] % 10_u128.pow(9);
    }
    println!("{}", a[n]);
}
