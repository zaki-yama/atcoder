use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut s: String,
        x_c: [(usize, char); q],
    }
    let original_abc_count = s.matches("ABC").count();
    let mut count = original_abc_count;
    let mut chars: Vec<char> = s.chars().collect();


    for i in 0..q {
        let query = x_c[i];
        let x = query.0 - 1;
        let c = query.1;

        let target = chars[x];

        match target {
            'A' => {
                if x < n - 2 && chars[x + 1] == 'B' && chars[x + 2] == 'C' {
                    count -= 1;
                }
            }
            'B' => {
                if x > 0 && x < n - 1 && chars[x - 1] == 'A' && chars[x + 1] == 'C' {
                    count -= 1;
                }
            }
            'C' => {
                if x > 1 && chars[x - 2] == 'A' && chars[x - 1] == 'B' {
                    count -= 1;
                }
            }
            _ => {}
        }

        match c {
            'A' => {
                if x < n - 2 && chars[x + 1] == 'B' && chars[x + 2] == 'C' {
                    count += 1;
                }
            }
            'B' => {
                if x > 0 && x < n - 1 && chars[x - 1] == 'A' && chars[x + 1] == 'C' {
                    count += 1;
                }
            }
            'C' => {
                if x > 1 && chars[x - 2] == 'A' && chars[x - 1] == 'B' {
                    count += 1;
                }
            }
            _ => {}
        }

        println!("{}", count);

        chars[x] = c;
    }
}
