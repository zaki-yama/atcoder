use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: usize,
        s: [String; n],
    }

    let mut all = vec![];
    let current = String::new();

    dfs(k, &s, current.clone(), &mut all);

    all.sort();
    println!("{}", all[x - 1]);
}

fn dfs(k: usize, s: &[String], current: String, all: &mut Vec<String>) {
    if k == 0 {
        all.push(current);
        return;
    }
    for t in s {
        dfs(k - 1, s, current.clone() + t, all);
    }
}
