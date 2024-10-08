use proconio::{input, marker::Chars};

fn convert_to_bit(chars: &Vec<char>) -> u32 {
    let bits: Vec<u32> = chars
        .iter()
        .map(|&ch| {
            if ch == 'o' {
                return 1;
            } else {
                return 0;
            };
        })
        .collect();
    println!("{:?}", bits);
    return 1;
}

fn main() {
    input! {
        n: usize,
        m: usize,
        mut s: [Chars; n],
    }

    let bits: Vec<u32> = s.iter().map(|chars| convert_to_bit(chars)).collect();
    bits.iter().for_each(|bit| {
        println!("{}", bit);
    });

    // n 個の売り場を選ぶ/選ばないの全探索
    // for bit in 0..(1 << n) {
    //     let mut selected: Vec<String> = Vec::new();
    //     for i in 0..n {
    //         if bit & (1 << i) != 0 {
    //             selected.push(s[i]);
    //         }
    //     }
    // }
}
