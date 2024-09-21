use proconio::input;

fn main() {
    input! {
        mut m: u32,
    }
    let mut ans = m;
    let mut a = Vec::new();
    for _i in 0..=20 {
        // mを超えない最大の3^xを見つける
        let x = find_max_power_of_three(ans);
        ans = ans - 3_u32.pow(x);
        a.push(x);

        if ans == 0 {
            println!("{}", a.len());
            // a を半角スペース区切りで出力（末尾は除く）
            println!(
                "{}",
                a.iter()
                    .map(|&x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            );
            return;
        }
    }
}

fn find_max_power_of_three(m: u32) -> u32 {
    for i in 0..=10 {
        if 3_u32.pow(i) > m {
            return i - 1;
        }
    }
    return 10;
}
