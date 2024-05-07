use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: i32,
        w: i32,
        board: [Chars; h],
    }

    for i in 0..h {
        for j in 0..w {
            if board[i as usize][j as usize] == '.' {
                continue;
            }
            // 隣接するマスに#があるかどうかを調べる
            if !is_adjacent_to_pound(&board, i, j, h, w) {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}

fn is_adjacent_to_pound(board: &Vec<Vec<char>>, i: i32, j: i32, h: i32, w: i32) -> bool {
    // 左
    if i - 1 >= 0 && board[(i - 1) as usize][j as usize] == '#' {
        return true;
    }
    // 右
    if i + 1 < w && board[(i + 1) as usize][j as usize] == '#' {
        return true;
    }
    // 上
    if j - 1 >= 0 && board[i as usize][(j - 1) as usize] == '#' {
        return true;
    }
    // 下
    if j + 1 < h && board[i as usize][(j + 1) as usize] == '#' {
        return true;
    }
    false
}
