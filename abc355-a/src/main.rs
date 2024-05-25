use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let candidates = vec![1, 2, 3];
    if a == b {
        println!("-1");
        return;
    }

    let answer = candidates.iter().position(|&x| x != a && x != b).unwrap();
    println!("{}", candidates[answer]);
}
