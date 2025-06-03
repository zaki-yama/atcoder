use proconio::input;

fn main() {
    input! {
        q: usize,
    }
    let mut menus: Vec<u32> = vec![];
    let mut customers_num = 0;

    for _ in 0..q {
        input! {
            query: usize,
        }

        if query == 1 {
            input! {
                menu: u32,
            }
            menus.push(menu);
        } else {
            customers_num += 1;
        }
    }
    for i in 0..customers_num {
        println!("{}", menus[i]);
    }
}
