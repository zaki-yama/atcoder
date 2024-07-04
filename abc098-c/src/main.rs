use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut chars: Chars,
    }

    let mut s = vec![];
    s.push(0);

    // s[i]: 1~i 番までで W を向いている人の数
    let mut count = 0;
    for &ch in &chars {
        if ch == 'W' {
            count += 1;
        }
        s.push(count);
    }

    // i 番目がリーダーだったとき、
    // 1~i-1 番目まで: W を向いている人が向く方向を変える => s[i-1] 人
    // i+1~n 番目まで: E を向いている人が向く方向を変える
    // => W を向いているのが s[n]-s[i] 人
    // => E を向いているのは {n-(i+1)+1} - (s[n]-s[i])
    let mut ans = n;
    for i in 1..=n {
        let target_num = s[i - 1] + n - i - (s[n] - s[i]);
        if ans > target_num {
            ans = target_num;
        }
    }
    println!("{}", ans);
}
