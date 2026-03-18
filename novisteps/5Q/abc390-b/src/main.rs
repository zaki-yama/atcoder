use proconio::input;

fn main() {
    input! {
        n: usize,
        A: [u64; n],
    }
    for i in 0..n - 2 {
        if A[i + 1] * A[i + 1] != A[i] * A[i + 2] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
