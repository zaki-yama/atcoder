use proconio::input;

fn main() {
    input! {
        n: u32,
        mut s_c: [(String, u32); n],
    }
    let (mut s, c): (Vec<String>, Vec<u32>) = s_c.into_iter().unzip();
    s.sort();

    let t: u32 = c.iter().sum();
    let index = (t % n) as usize;
    println!("{}", s[index]);
}
