use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i32; m],
        x: [[i32; m]; n]
    }
    let mut ans = a.clone();
    x.iter()
        .for_each(|xi| ans = ans.iter().zip(xi.iter()).map(|(p, q)| p - q).collect());
    let result = if ans.iter().all(|&x| x <= 0) {
        "Yes"
    } else {
        "No"
    };
    println!("{}", result);
}
