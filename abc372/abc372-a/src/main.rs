use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let ans: String = s.chars().filter(|ch| *ch != '.').collect();
    println!("{}", ans);
}
