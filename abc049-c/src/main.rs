// -*- coding:utf-8-unix -*-

use proconio::input;

const DIVIDE: [&str; 4] = ["dream", "dreamer", "erase", "eraser"];

fn main() {
    input! {
        s: String,
    }

    // 後ろから解くかわりにすべての文字列を「左右反転」する
    let reverse = s.chars().rev().collect::<String>();
    let reverse_divide = DIVIDE
        .iter()
        .map(|&d| d.chars().rev().collect::<String>())
        .collect::<Vec<String>>();

    let mut can = true;
    let mut i = 0;
    while i < reverse.len() {
        let mut can_divide = false;
        for d in reverse_divide.iter() {
            if reverse[i..].starts_with(d) {
                can_divide = true;
                i += d.len();
                break;
            }
        }
        if !can_divide {
            can = false;
            break;
        }
    }
    let result = if can { "YES" } else { "NO" };
    println!("{}", result);
}
