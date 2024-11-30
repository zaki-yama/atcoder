use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        s: String,
    }
    let cookies = s.chars().filter(|&ch| ch == '@').count();
    println!("{}", n - cookies + d);
}
