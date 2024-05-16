use proconio::input;

fn main() {
    input! {
        n: usize,
        heights: [i32; n],
    }
    let highest = heights[0];
    for i in 1..n {
        if heights[i] > highest {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}
