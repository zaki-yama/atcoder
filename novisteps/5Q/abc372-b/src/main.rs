use proconio::input;

fn main() {
    input! {
        mut m: u32,
    }

    let mut ans = vec![];
    for k in 0..=10 {
        let res = m % 3;
        for _ in 0..res {
            ans.push(k);
        }
        m /= 3;
        if m == 0 {
            println!("{}", ans.len());
            println!(
                "{}",
                ans.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            );
            return;
        }
    }
}
