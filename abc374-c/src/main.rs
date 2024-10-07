use proconio::input;

fn main() {
    input! {
        n: usize,
        k: [u32; n],
    }

    let sum: u32 = k.iter().sum();
    let mut ans = sum;
    for bit in 0..(1 << n) {
        let mut group_a_sum = 0;
        // bitが立っている = グループAとみなす
        for i in 0..n {
            if bit & (1 << i) != 0 {
                group_a_sum += k[i];
            }
        }
        // println!("{:06b}: {}", bit, group_a_sum);

        let group_b_sum = sum - group_a_sum;
        if group_a_sum.max(group_b_sum) < ans {
            ans = group_a_sum.max(group_b_sum);
        }
    }
    println!("{}", ans);
}
