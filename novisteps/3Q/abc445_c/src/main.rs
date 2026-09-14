use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
    }

    for i in 0..n {
        a[i] -= 1;
    }

    let mut ans: Vec<u32> = vec![0u32; n];

    for i in (0..n).rev() {
        let current = a[i];
        if current == i as u32 {
            ans[i] = i as u32;
        } else {
            ans[i] = ans[a[i] as usize];
        }
    }

    println!(
        "{}",
        ans.iter()
            .map(|v| (v + 1).to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
