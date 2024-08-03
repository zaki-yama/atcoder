use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [u64; N],
    }
    let mut b = A.clone();
    b.sort_by(|a, b| b.cmp(a));
    let ans = A.iter().position(|&x| x == b[1]).unwrap();
    println!("{}", ans + 1);
}
