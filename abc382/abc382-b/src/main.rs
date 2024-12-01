use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        s: String,
    }
    let mut chars: Vec<char> = s.chars().collect();

    for _ in 0..d {
        if let Some(pos) = chars.iter().rposition(|&ch| ch == '@') {
            chars[pos] = '.';
        } else {
            break;
        }
    }

    let result: String = chars.iter().collect();
    println!("{}", result);
}
