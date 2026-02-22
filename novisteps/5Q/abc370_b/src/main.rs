use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut a = vec![];
    for i in 0..n {
        input! {
            row: [usize; i + 1],
        }
        a.push(row);
    }

    let mut current = 0;
    for i in 0..n {
        if current >= i {
            current = a[current][i];
        } else {
            current = a[i][current];
        }
        current -= 1;
    }
    println!("{}", current + 1);
}
