use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    let mut h = 0;
    for i in 0..n {
        if h < a[i] {
            h = a[i] - 1;
        }
        if h == 0 {
            println!("{}", i + 1);
            return;
        }
        h -= 1;
    }
    println!("{}", n);
}
