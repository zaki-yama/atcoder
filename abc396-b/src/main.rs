use proconio::{input, marker::Chars};

fn main() {
    input! {
        q: usize,
    }
    let mut cards = vec![0; 100];
    for _i in 0..q {
        input! {
            query: u32,
        }
        match query {
            1 => {
                input! {
                    num: u32,
                }
                cards.push(num);
            }
            2 => {
                println!("{}", cards.pop().unwrap());
            }
            _ => {}
        }
    }
}
