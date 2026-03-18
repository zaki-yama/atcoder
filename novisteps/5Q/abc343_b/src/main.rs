use proconio::input;

fn main() {
    input! {
        n: usize,
        A: [[u32; n]; n]
    }
    for i in 0..n {
        let mut nums = vec![];
        for j in 0..n {
            if A[i][j] == 1 {
                nums.push(j + 1);
            }
        }
        println!(
            "{}",
            nums.iter()
                .map(|n| { n.to_string() })
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}
