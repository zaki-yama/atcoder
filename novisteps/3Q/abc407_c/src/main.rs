use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let nums: Vec<u32> = s.iter().filter_map(|c| c.to_digit(10)).collect();
    let n = nums.len();
    if n == 1 {
        println!("{}", nums[0] + 1);
        return;
    }

    let mut ans = 0;
    for i in 0..n - 1 {
        if nums[i] < nums[i + 1] {
            ans += 10 - (nums[i + 1] - nums[i]);
        } else if nums[i] > nums[i + 1] {
            ans += nums[i] - nums[i + 1];
        }
        // println!("num: {}, ans: {}", nums[i], ans);
    }

    ans += nums[n - 1];
    ans += nums.len() as u32;
    println!("{}", ans);
}
