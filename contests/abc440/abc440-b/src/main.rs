use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [u32; n],
    }

    let mut copy = t.clone();
    copy.sort();

    println!(
        "{} {} {}",
        t.iter().position(|&x| x == copy[0]).unwrap() + 1,
        t.iter().position(|&x| x == copy[1]).unwrap() + 1,
        t.iter().position(|&x| x == copy[2]).unwrap() + 1
    );
}
