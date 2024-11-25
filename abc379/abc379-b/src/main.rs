use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: String
    }
    let mut counter = 0;
    let mut ans = 0;
    s.chars().for_each(|ch| {
        if ch == 'O' {
            counter += 1;
            if counter == k {
                ans += 1;
                counter = 0;
            }
        } else {
            counter = 0;
        }
    });
    println!("{}", ans);
}
