use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; q],
    }

    let mut v = vec![0; n + 2];
    let mut count = 0;
    for i in 0..q {
        let idx = a[i];
        match v[idx] {
            0 => {
                if v[idx - 1] == 0 && v[idx + 1] == 0 {
                    count += 1;
                }
                if v[idx - 1] == 1 && v[idx + 1] == 1 {
                    count -= 1;
                }
            }
            1 => {
                if v[idx - 1] == 0 && v[idx + 1] == 0 {
                    count -= 1;
                }
                if v[idx - 1] == 1 && v[idx + 1] == 1 {
                    count += 1;
                }
            }
            _ => panic!(),
        }
        println!("{}", count);
        v[a[i]] = v[a[i]] ^ 1;
    }
}
