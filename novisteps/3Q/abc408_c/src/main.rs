use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        l_r: [(u32, u32); m],
    }

    let mut diff: Vec<i32> = vec![0; n + 1];
    for i in 0..m {
        let (l, r) = l_r[i];
        diff[(l - 1) as usize] += 1;
        diff[r as usize] -= 1;
    }

    let mut min = i32::MAX;
    let mut current = 0;
    for i in 0..n {
        current += diff[i];
        if current < min {
            min = current;
        }
    }
    println!("{}", min);
}
