use proconio::input;

fn main() {
    input! {
        n: u64,
        m: u64,
        a: [u64; n],
    }
    // sum(a) < m なら infinite
    if a.iter().sum::<u64>() <= m {
        println!("infinite");
        return;
    }

    // 二分探索で解を探す
    let mut ok = 0u64;
    let mut ng = 1_000_000_000u64;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        // a の各要素について、mid より大きければ mid を、そうでなければ a[i] を足す
        let sum: u64 = a.iter().map(|&x| std::cmp::min(mid, x)).sum();
        if sum <= m {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}
