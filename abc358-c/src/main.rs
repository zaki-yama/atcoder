use proconio::{input, marker::Chars};

fn convert_to_bit(chars: &Vec<char>) -> u32 {
    // 'o' -> 1, 'x' -> 0 で 11010 のようなビットに変換する
    let bit = chars
        .iter()
        // 左のビットから、<< で1ビットシフトし、次のビットを足す
        .fold(0, |acc, &ch| (acc << 1) | if ch == 'o' { 1 } else { 0 });
    return bit;
}

fn main() {
    input! {
        n: u32,
        m: u32,
        mut s: [Chars; n],
    }

    let bit_vec: Vec<u32> = s.iter().map(|chars| convert_to_bit(chars)).collect();

    let mut ans = n;
    // n 個の売り場を選ぶ/選ばないの全探索
    for bit in 0..(1 << n) {
        let mut selected: Vec<u32> = Vec::new();
        for i in 0..n {
            if bit & (1 << i) != 0 {
                selected.push(bit_vec[i as usize]);
            }
        }

        // 選択した売り場の組み合わせですべての味を購入できるかチェック
        // selected の要素のビット和を取り、全ての味が購入できるかチェック
        let result = selected.iter().fold(0, |acc, &bit| acc | bit);
        if result.count_ones() == m && selected.len() < ans as usize {
            ans = selected.len() as u32;
        }
    }
    println!("{}", ans);
}
