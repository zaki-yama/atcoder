use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],  // Vec<(i32, i32, i32)>
    }
    for i in 0..n - 2 {
        if a[i] == a[i + 1] && a[i] == a[i + 2] {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
