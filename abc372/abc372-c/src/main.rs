use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut s: String,
        x_c: [(usize, char); q],
    }
    // let mut ans = Vec::new();
    for i in 0..q {
        let query = x_c[i];
        s = s
            .chars()
            .enumerate()
            .map(|(i, ch)| if i == query.0 - 1 { query.1 } else { ch })
            .collect();
        // println!("{}: {}", i, s);
        let abc_count = s.matches("ABC").count();
        println!("{}", abc_count);
    }
}
