use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut v = vec![];
    for i in 0..n {
        v.push((n - i).to_string());
    }
    println!("{}", v.join(","));
}
