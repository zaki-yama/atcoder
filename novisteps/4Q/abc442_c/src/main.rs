use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a_b: [(u32, u32); m],
    }

    let mut stakeholders: HashMap<u32, Vec<u32>> = HashMap::new();
    a_b.iter().for_each(|(ai, bi)| {
        let val = stakeholders.entry(*ai).or_insert(vec![]);
        val.push(*bi);
        let val = stakeholders.entry(*bi).or_insert(vec![]);
        val.push(*ai);
    });

    let mut combinations: Vec<i64> = vec![];
    for i in 1..=n {
        let stakeholders = stakeholders.entry(i as u32).or_insert(vec![]);
        let nums: i64 = n as i64 - 1 - stakeholders.len() as i64;
        // 候補が3人以上いればxC3で、3人未満だったら0
        combinations.push(if nums < 3 {
            0
        } else {
            (nums * (nums - 1) * (nums - 2)) / 6
        });
    }

    println!(
        "{}",
        combinations
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
}
