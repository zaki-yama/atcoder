use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: u32,
    }

    // 正しい鍵を1、ダミー鍵を0で表現したN個の鍵の全組み合わせ（2^N通り）
    let patterns = generate_binary_combinations(n);

    let mut test_cases: Vec<Vec<u32>> = vec![];
    let mut test_results: Vec<char> = vec![];

    for i in 0..m {
        input! {
            c: usize,
            a: [u32; c],
            r: char,
        }
        test_cases.push(a);
        test_results.push(r);
    }
    // println!("{:?}", patterns);
    // println!("{:?}", test_cases);
    // println!("{:?}", test_results);

    let mut ans = 0;
    for i in 0..patterns.len() {
        let pattern = &patterns[i];
        let mut is_valid_pattern = true;
        // println!("----------------\npattern: {:?}", pattern);

        // 全てのテストケースと比較し、矛盾がなければansをカウントアップ
        for j in 0..m {
            let test_case = &test_cases[j];
            // println!("test_case: {:?}", test_case);

            let mut sum = 0;
            test_case.iter().for_each(|index| {
                sum += pattern[(index - 1) as usize];
            });
            // sum が正しい鍵の本数になる

            // println!("sum: {}", sum);
            let expected = if sum >= k { 'o' } else { 'x' };
            let actual = test_results[j];
            // println!("expected: {}, actual: {}", expected, actual);
            if expected != actual {
                is_valid_pattern = false;
                break;
            }
        }
        if is_valid_pattern {
            ans += 1;
        }
    }
    println!("{}", ans);
}

fn generate_binary_combinations(n: usize) -> Vec<Vec<u32>> {
    let num_combinations = 1 << n; // 2^n
    let mut combinations = Vec::with_capacity(num_combinations);

    for i in 0..num_combinations {
        let mut binary_vec = Vec::with_capacity(n);
        for j in (0..n).rev() {
            binary_vec.push((i >> j & 1) as u32);
        }
        combinations.push(binary_vec);
    }

    combinations
}
