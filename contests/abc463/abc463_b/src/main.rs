use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        x: char,
        seats: [Chars; n],
    }
    let idx = match x {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4,
        _ => panic!(),
    };

    for i in 0..n {
        if seats[i][idx] == 'o' {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
