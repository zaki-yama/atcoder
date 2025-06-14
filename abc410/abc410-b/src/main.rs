use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        x: [u32; q],
    }

    let mut ans: Vec<u32> = vec![];
    let mut cnt: Vec<u32> = vec![0; n];

    for i in 0..q {
        if x[i] != 0 {
            ans.push(x[i]);
            cnt[(x[i] - 1) as usize] += 1;
        } else {
            let min_index = find_min_index(&cnt);
            ans.push(min_index + 1);
            cnt[min_index as usize] += 1;
        }
    }
    println!(
        "{}",
        ans.iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

fn find_min_index(x: &Vec<u32>) -> u32 {
    let mut res: u32 = 0;
    let mut min = 101;
    for i in 0..x.len() {
        if min > x[i] {
            min = x[i];
            res = i as u32;
        }
    }
    return res;
}
