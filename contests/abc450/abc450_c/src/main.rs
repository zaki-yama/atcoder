use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut checked: Vec<Vec<bool>> = vec![vec![false; w]; h];
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' && !checked[i][j] {
                if !bfs(i, j, h, w, &s, &mut checked) {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}

fn bfs(
    si: usize,
    sj: usize,
    h: usize,
    w: usize,
    s: &[Vec<char>],
    checked: &mut Vec<Vec<bool>>,
) -> bool {
    // (si, sj) を含む白マスの領域が最外周のマスを含むか
    let mut out = false;
    let mut q = VecDeque::from([(si, sj)]);
    checked[si][sj] = true;
    while let Some((i, j)) = q.pop_front() {
        if i == 0 || i == h - 1 || j == 0 || j == w - 1 {
            out = true;
        }
        for (di, dj) in [(0i64, 1i64), (0, -1), (1, 0), (-1, 0)] {
            let ii = i as i64 + di;
            let jj = j as i64 + dj;
            if ii >= 0 && ii < h as i64 && jj >= 0 && jj < w as i64 {
                let ii = ii as usize;
                let jj = jj as usize;
                if s[ii][jj] == '.' && !checked[ii][jj] {
                    q.push_back((ii, jj));
                    checked[ii][jj] = true;
                }
            }
        }
    }
    out
}
