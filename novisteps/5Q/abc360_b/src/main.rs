use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }
    let chars: Vec<char> = s.chars().collect();
    for w in 1..s.len() {
        for c in 0..w {
            let mut now = String::new();
            for i in (c..s.len()).step_by(w) {
                now.push(chars[i]);
            }
            if now == t {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
