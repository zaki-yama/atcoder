use proconio::input;

fn main() {
    input! {
        n: u32,
        k: u32,
        mut a: [u128; n],
    }

    let y = 10u128.pow(k);
    let mut ans = 1;

    for &ai in &a {
        ans *= ai;
        if ans >= y {
            ans = 1; // リセットの意図を明確にするコメントを追加
        }
    }

    println!("{}", ans);
}
