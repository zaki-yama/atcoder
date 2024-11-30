use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        s: String,
    }
    let cookies = s.chars().filter(|&ch| ch == '@').count();
    let cookies_rest = cookies - d;
    if cookies_rest == 0 {
        println!("{}", ".".repeat(n));
        return;
    }
    let index = find_last_cookie_index(&s, n, cookies_rest);
    let head: String = s.chars().take(index).collect();
    let dots = ".".repeat(n - index);

    println!("{}{}", head, dots);
}

fn find_last_cookie_index(s: &str, n: usize, num_cookies: usize) -> usize {
    let mut chars = s.chars();
    let mut count = 0;
    for i in 0..n {
        let ch = chars.nth(0).unwrap();
        if ch == '@' {
            count += 1;
            if count > num_cookies {
                return i;
            }
        }
    }
    return n - 1;
}
