use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = vec![vec!['.'; n]; n];
    for i in (0..=n / 2).step_by(2) {
        for j in 0..n - 2 * i {
            ans[i][i + j] = '#';
            ans[i + j][i] = '#';
            ans[i + j][n - i - 1] = '#';
            ans[n - i - 1][i + j] = '#';
        }
    }

    for i in 0..n {
        for j in 0..n {
            print!("{}", ans[i][j]);
        }
        println!("");
    }
}
