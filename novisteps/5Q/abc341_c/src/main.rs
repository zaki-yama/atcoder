use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        t: Chars,
        s: [Chars; h],
    }

    let mut ans = 0;
    for i in 1..h - 1 {
        for j in 1..w - 1 {
            'check: {
                let mut x = i;
                let mut y = j;
                if s[x][y] == '#' {
                    break 'check;
                }
                for k in 0..n {
                    (x, y) = match t[k] {
                        'L' => (x, y - 1),
                        'R' => (x, y + 1),
                        'U' => (x - 1, y),
                        'D' => (x + 1, y),
                        _ => panic!(),
                    };
                    let next = s.get(x).and_then(|row| row.get(y)).copied().unwrap_or('#');
                    if next == '#' {
                        break 'check;
                    }
                }
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
