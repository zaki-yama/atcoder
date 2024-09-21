use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        r: [u32; n],
    }
    let max = 5; // 各要素の最大値
    let mut current = Vec::new();
    let mut possible_sequences = Vec::new();

    let mut result = Vec::new();

    generate_sequences(n, max, &mut current, &mut possible_sequences);

    possible_sequences.iter().for_each(|sequence| {
        if meets_condition(sequence, k as u32, &r) {
            result.push(sequence.clone());
        }
    });


    if result.len() == 0 {
        println!("");
        return;
    } else {
        result.iter().for_each(|sequence| {
            let seq_str: Vec<String> = sequence.iter().map(|&x| x.to_string()).collect();
            println!("{}", seq_str.join(" "));
        });
    }
}

fn generate_sequences(n: usize, max: u32, current: &mut Vec<u32>, result: &mut Vec<Vec<u32>>) {
    if current.len() == n {
        result.push(current.clone());
        return;
    }

    for i in 1..=max {
        current.push(i);
        generate_sequences(n, max, current, result);
        current.pop();
    }
}
fn meets_condition(target: &Vec<u32>, k: u32, r: &Vec<u32>) -> bool {
    if target.iter().sum::<u32>() % k != 0 {
        return false;
    }
    for i in 0..target.len() {
        if target[i] > r[i] {
            return false;
        }
    }
    true
}
