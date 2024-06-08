use proconio::input;

fn main() {
    input! {
        S: String,
    }
    let lowercase_count = S.chars().filter(|ch| ch.is_lowercase()).count();
    // 小文字が過半数なら全部小文字で出力
    if lowercase_count > (S.len() - 1) / 2 {
        println!("{}", S.to_lowercase());
    } else {
        println!("{}", S.to_uppercase());
    }
}
