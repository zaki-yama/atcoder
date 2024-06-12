use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut index = 0;
    for i in 1..=n {
        // println!("{}: {} {}", i, index, a[index]);
        if a[index] == 2 {
            println!("{}", i);
            return;
        }
        index = (a[index] - 1) as usize;
    }
    println!("-1");
}
