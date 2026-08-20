use proconio::input;

fn main() {
    input! {
        n: usize,
        r: usize,
        l: [i64; n],
    }

    let mut x = n;
    let mut y = 0;
    for i in 0..n {
        if l[i] == 0 {
            x = i;
            break;
        }
    }
    for i in (0..n).rev() {
        if l[i] == 0 {
            y = i;
            break;
        }
    }

    let mut open: i64 = 0;
    for i in 0..n {
        if l[i] == 0 {
            open += 1;
        }
    }

    let mut t: i64 = 0;
    if x < r {
        for i in x + 1..r {
            if l[i] == 1 {
                t += 1;
            }
        }
    }
    if y >= r {
        for i in r..y {
            if l[i] == 1 {
                t += 1;
            }
        }
    }

    println!("{}", 2 * t + open);
}
