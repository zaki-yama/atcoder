use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u64; n],
    }

    let mut cursor = 0;
    for i in 0..q {
        // println!("---------{}", i);
        // println!("cursor is now: {}", cursor);
        input! {
            t: usize,
        }

        match t {
            1 => {
                input! {
                    c: usize,
                }

                cursor = (cursor + c) % n;
                // println!("1: new cursor is: {}", cursor);
            }
            2 => {
                input! {
                    l: usize,
                    r: usize,
                }

                let l = (l + cursor - 1) % n;
                let r = (r + cursor - 1) % n;
                let sum: u64 = if l <= r {
                    a[l..=r].iter().copied().sum()
                } else {
                    a[l..n].iter().copied().sum::<u64>() + a[0..=r].iter().copied().sum::<u64>()
                };
                println!("{}", sum);
            }
            _ => panic!(),
        }
    }
}
