use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
    }

    let mut stack = vec![];
    for &ai in &a {
        stack.push(ai);
        if stack.len() >= 4 {
            let last4 = &stack[stack.len() - 4..];
            if last4.iter().all(|&x| x == last4[0]) {
                stack.pop();
                stack.pop();
                stack.pop();
                stack.pop();
            }
        }
    }
    println!("{}", stack.len());
}
