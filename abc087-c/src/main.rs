use proconio::input;

fn main() {
    input! {
        n: usize,
        board: [[i32; n]; 2],
    }
    let mut candies = vec![];

    for i in 0..n {
        // println!("{}", i);
        let mut sum = 0;
        for j in 0..i {
            // println!("1: {}", j);
            sum += board[0][j];
        }

        sum += board[0][i];
        sum += board[1][i];

        for j in i + 1..n {
            // println!("2: {}", j);
            sum += board[1][j];
        }

        candies.push(sum);
    }

    println!("{}", candies.iter().max().unwrap());
}
