use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut ans = 0;
    let mut total = 0;
    a.iter().for_each(|&ai| {
        if k - total >= ai {
            total += ai;
        } else {
            ans += 1;
            total = ai;
        }
    });
    println!("{}", ans + 1);
}
