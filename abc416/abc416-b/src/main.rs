use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }

    let mut can_change = true;
    for i in 0..s.len() {
        if s[i] == '.' && can_change {
            s[i] = 'o';
            can_change = false;
        } else if s[i] == '#' {
            can_change = true;
        }
    }

    println!("{}", s.iter().collect::<String>());
}
