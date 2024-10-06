use proconio::input;

fn maskbit() {
    let bit = 0b11001011;
    let mask = 0b11110000;

    // mask 部分のフラグをまとめて立てる
    let mut ex1 = bit;
    ex1 |= mask;
    println!("{:08b}", ex1);

    // mask 部分のフラグをまとめて消す
    let mut ex2 = bit;
    ex2 &= !mask;
    println!("{:08b}", ex2);
}

/// bit 全探索
fn bit_full_search() {
    let n = 5;
    for bit in 0..(1 << n) {
        println!("{}", bit);
    }
    // for (int bit = 0; bit < (1<<N); ++bit) {

    // }
}

fn main() {
    input! {
        abcd: String,
    }
    // abcd をa,b,c,dに分割し int にする

    let chars: Vec<char> = abcd.chars().collect();
    let a = chars[0].to_digit(10).unwrap();
    let b = chars[1].to_digit(10).unwrap();
    let c = chars[2].to_digit(10).unwrap();
    let d = chars[3].to_digit(10).unwrap();

    // n=3ビットでループ
    for bit in 0..(1 << 3) {
        // 各bitについて、1なら+、0なら-

        // abcdと↑の符号の組み合わせて計算

        let ans;
        if (ans == 7) {
            println!("");
            return;
        }
    }
}
