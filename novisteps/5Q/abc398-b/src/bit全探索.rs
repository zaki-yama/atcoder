use proconio::input;

fn main() {
    input! {
        a: [u32; 7],
    }

    // 7 枚のカードそれぞれを選ぶ/選ばないのビット全探索
    for bit in 0..(1 << 7) {
        let mut selected = Vec::new();
        // j 番目のビットが立っている = j 番目のカードを選んだ
        for j in 0..7 {
            if bit & (1 << j) != 0 {
                selected.push(a[j]);
            }
        }

        // 5 枚選んでなければ終了
        if selected.len() != 5 {
            continue;
        }

        selected.sort();

        if (selected[0] == selected[1]
            && selected[1] == selected[2]
            && selected[2] != selected[3]
            && selected[3] == selected[4])
            || (selected[0] == selected[1]
                && selected[1] != selected[2]
                && selected[2] == selected[3]
                && selected[3] == selected[4])
        {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
