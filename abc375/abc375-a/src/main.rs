use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String
    }

    let mut ans = 0;
    if n < 2 {
        println!("{}", ans);
        return;
    }

    let chars: Vec<char> = s.chars().collect();
    for i in 0..n - 2 {
        if chars[i] == '#' && chars[i + 1] == '.' && chars[i + 2] == '#' {
            ans = ans + 1;
        }
    }
    println!("{}", ans);
}
