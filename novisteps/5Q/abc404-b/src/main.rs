use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
        mut t: [Chars; n],
    }

    let mut ans = vec![0, 1, 2, 3];
    ans[0] = count_different_cells(n, &s, &t);

    rotate_90(n, &mut s);
    ans[1] += count_different_cells(n, &s, &t);

    rotate_90(n, &mut s);
    ans[2] += count_different_cells(n, &s, &t);

    rotate_90(n, &mut s);
    ans[3] += count_different_cells(n, &s, &t);

    println!("{}", ans.iter().min().unwrap());
}

fn rotate_90(n: usize, matrix: &mut Vec<Vec<char>>) {
    // 転置
    for i in 0..n {
        for j in i + 1..n {
            let temp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = temp;
        }
    }
    // 各行を反転
    for row in matrix.iter_mut() {
        row.reverse();
    }
}

fn count_different_cells(n: usize, s: &Vec<Vec<char>>, t: &Vec<Vec<char>>) -> u32 {
    let mut result = 0;

    for i in 0..n {
        for j in 0..n {
            if s[i][j] != t[i][j] {
                result += 1;
            }
        }
    }
    return result;
}
