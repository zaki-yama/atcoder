use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [i32; n],  // Vec<(i32, i32, i32)>
        mut b: [i32; m],  // Vec<(i32, i32, i32)>
    }
    let mut c = a.clone();
    c.append(&mut b);
    c.sort();
    for w in c.windows(2) {
        if a.contains(&w[0]) && a.contains(&w[1]) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
