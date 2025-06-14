use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut a: Vec<u32> = (1..=n).map(|x| x as u32).collect();
    let mut offset = 0;
    for _ in 0..q {
        input! {
            query: usize,
        }
        match query {
            1 => {
                input! {
                    p: usize,
                    x: u32
                }
                let index = ((p - 1) + offset) % n;
                a[index] = x;
            }
            2 => {
                input! {
                    p: usize,
                }
                let index = ((p - 1) + offset) % n;
                println!("{}", a[index]);
            }
            3 => {
                input! {
                    k: usize,
                }
                offset += k;
                offset %= n;
            }
            _ => panic!(),
        }
    }
}
