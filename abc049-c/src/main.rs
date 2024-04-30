// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
    }
    if is_match(&s) {
        println!("YES")
    } else {
        println!("NO")
    };
}

fn is_match(s: &str) -> bool {
    if s.len() == 0 {
        return true;
    }

    if s.starts_with("dream") || s.starts_with("erase") {
        if s.starts_with("dreamer") {
            if is_match(&s[7..]) {
                return true;
            }
        } else if s.starts_with("eraser") {
            if is_match(&s[6..]) {
                return true;
            }
        }
        return is_match(&s[5..]);
    }
    return false;
}
