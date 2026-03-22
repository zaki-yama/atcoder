use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut c = vec![];
    for i in 0..n - 1 {
        input! {
            ci: [u32; n-i-1],
        }
        c.push(ci);
    }

    // println!("{:?}", c);
    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            for k in j + 1..n {
                // println!("({}, {}, {})", i, j, k);
                let c_ab = c[i][j - i - 1];
                // println!("c_ab {}", c_ab);
                let c_bc = c[j][k - j - 1];
                // println!("c_bc {}", c_bc);
                let c_ac = c[i][k - i - 1];
                // println!("c_ac {}", c_ac);
                if c_ab + c_bc < c_ac {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
