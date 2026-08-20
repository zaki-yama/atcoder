use std::{
    collections::{HashMap, HashSet},
    println,
};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
    }

    let mut perms: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut admins = vec![0; n];
    for i in 0..q {
        input! {
            query_type: usize,
            x: usize,
        }

        match query_type {
            1 => {
                input! {
                    y: usize,
                }

                perms.entry(x).or_default().insert(y);
            }
            2 => {
                admins[x - 1] = 1;
            }
            3 => {
                input! {
                    y: usize,
                }

                if admins[x - 1] == 1 || perms.get(&x).map_or(false, |set| set.contains(&y)) {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
            _ => panic!(),
        }
    }
}
