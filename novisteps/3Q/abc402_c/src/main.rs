use std::{collections::HashMap, println};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut counts = HashMap::new();
    let mut ingredients_to_menu: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..m {
        input! {
            k: usize, // menu(+1する)
            a: [usize; k], // ingredients
        }
        counts.insert(i + 1, a.len());
        for j in 0..k {
            ingredients_to_menu
                .entry(a[j])
                .or_insert_with(Vec::new)
                .push(i + 1);
        }
    }

    input! {
        b: [usize; n],
    }

    let mut ans = 0;
    for i in 0..n {
        let ingredient = b[i];
        // println!("ingredient: {}", ingredient);

        ingredients_to_menu
            .get(&ingredient)
            .unwrap_or(&vec![])
            .iter()
            .for_each(|&menu| {
                let new_count = counts.get(&menu).unwrap() - 1;
                if new_count == 0 {
                    ans += 1;
                }
                counts.insert(menu, new_count);
            });

        println!("{}", ans);
    }
}
