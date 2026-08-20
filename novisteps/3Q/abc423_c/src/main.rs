use proconio::input;

fn main() {
    input! {
        n: usize,
        r: usize,
        mut l: [u32; n],
    }

    if l.iter().all(|&x| x == 1) {
        println!("0");
        return;
    }

    // 左端を探す
    let mut left = 0;
    while left < r {
        if l[left] == 0 {
            break;
        }
        left += 1;
    }
    // println!("左端を探す: {}", left);

    // 右端を探す
    let mut right = n - 1;
    while right >= r {
        if l[right] == 0 {
            break;
        }
        right -= 1;
    }
    // println!("右端を探す: {}", right);

    let mut sum = 0;
    // 出発点rから左に走査
    // println!("    // 出発点rから左に走査");
    for i in left..r {
        // println!("{}: {}", i, l[i]);
        sum += l[i] + 1;
    }

    // 出発点rから右に走査
    // println!("    // 出発点rから右に走査");
    for i in r..=right {
        // println!("{}: {}", i, l[i]);
        sum += l[i] + 1;
    }

    println!("{}", sum);
}
