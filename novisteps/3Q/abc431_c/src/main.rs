use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        mut h: [u32; n],
        mut b: [u32; m],
    }
    h.sort();
    b.sort();

    let mut cursor = 0;
    let mut cnt = 0;
    for i in 0..n {
        while cursor < b.len() {
            if h[i] <= b[cursor] {
                cnt += 1;
                if cnt == k {
                    println!("Yes");
                    return;
                }
                cursor += 1;
                break;
            }
            cursor += 1;
        }
    }
    println!("No");
}
