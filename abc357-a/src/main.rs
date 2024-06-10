use proconio::input;

fn main() {
    input! {
        N: usize,
        M: i32,
        H: [i32; N],
    }
    let mut sum = 0;
    for i in 0..N {
        sum += H[i];
        if sum > M {
            println!("{}", i);
            return;
        }
    }
    println!("{}", N);
}
