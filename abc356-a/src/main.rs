use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
    }
    let mut ans: Vec<usize> = vec![];
    for i in 1..=n {
        if l <= i && i <= r {
            ans.push(r - (i - l));
        } else {
            ans.push(i);
        }
    }
    let ans = ans
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", ans);
}
