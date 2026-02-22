use proconio::input;

fn main() {
    input! {
        Q: usize,
    }
    let mut stack = vec![];
    for _ in 0..Q {
        input! {
            query: u32,
        }
        if query == 1 {
            input! {
                value: u32,
            }
            stack.push(value);
        } else {
            println!("{}", stack.pop().unwrap_or(0));
        }
    }
}
