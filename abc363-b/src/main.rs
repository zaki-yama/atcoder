use proconio::input;

fn main() {
    input! {
        n: i32,
        t: i32,
        p: usize,
        mut l: [i32; n],
    }
    l.sort_by(|a, b| b.cmp(a));
    let ans: i32 = std::cmp::max(t - l[p - 1], 0);
    println!("{}", ans);
}
