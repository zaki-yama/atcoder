use proconio::input;

fn main() {
    input! {
        mut h: usize,
        mut w: usize,
        q: usize,
    }
    for i in 0..q {
        input! {
            t: usize,
            amount : usize,
        }
        match t {
            1 => {
                println!("{}", amount * w);
                h -= amount;
            }
            2 => {
                println!("{}", amount * h);
                w -= amount;
            }
            _ => panic!(),
        }
    }
}
