use std::char::from_digit;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: i32,
        w: i32,
        board: [Chars; h],
    }
    let mut result = board.clone();

    for i in 0..h {
        for j in 0..w {
            if board[i as usize][j as usize] == '#' {
                result[i as usize][j as usize] = '#';
                continue;
            }
            let mut count = 0;
            for l in i - 1..=i + 1 {
                if l < 0 || l >= h {
                    continue;
                }
                for m in j - 1..=j + 1 {
                    if m < 0 || m >= w {
                        continue;
                    }
                    if board[l as usize][m as usize] == '#' {
                        count += 1;
                    }
                }
            }
            result[i as usize][j as usize] = from_digit(count, 10).unwrap();
        }
    }
    result.iter().for_each(|row| {
        println!("{}", row.iter().collect::<String>());
    });
}
