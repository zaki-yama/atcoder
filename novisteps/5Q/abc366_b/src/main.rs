use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let (max_idx, max_len) = s
        .iter()
        .enumerate()
        .max_by_key(|(_, chars)| chars.len())
        .map(|(i, chars)| (i, chars.len()))
        .unwrap();

    for j in 0..max_len {
        let max_row_index = find_max_row_index(&s, j, n);
        for i in 0..n {
            let val = s[n - i - 1].get(j);
            if val.is_some() {
                print!("{}", val.unwrap());
            } else {
                if i <= max_row_index {
                    print!("*");
                }
            }
        }
        println!();
    }
}

fn find_max_row_index(s: &[Vec<char>], j: usize, n: usize) -> usize {
    let mut res = 0;
    for i in 0..n {
        if s[n - i - 1].get(j).is_some() {
            res = i;
        }
    }
    return res;
}
