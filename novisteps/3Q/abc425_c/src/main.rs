use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u64; n],
    }

    let mut s = vec![0; n];
    s[0] = a[0];
    for i in 1..n {
        s[i] = s[i - 1] + a[i];
    }
    // println!("{:?}", s);

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
                let sl = if l == 0 { 0 } else { s[l - 1] };
                let sum: u64 = if l <= r {
                    s[r] - sl
                } else {
                    s[n - 1] - sl + s[r]
                };
                println!("{}", sum);
            }
            _ => panic!(),
        }
    }
}
