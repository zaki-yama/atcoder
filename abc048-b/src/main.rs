use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        x: u64,
    }

    let mut ans = 0;
    let mut tmp = x;
    while tmp <= b {
        if tmp >= a {
            ans += 1;
        }
        tmp += x;
    }
    println!("{}", ans);
}
