#![allow(non_snake_case)]
use proconio::input;
use std::collections::HashMap;

/*
色の変化した日を管理するのが良い
色が消えた & 色が増えた として管理
全体の色を hashmap で管理。　(color: num bird)

*/
fn main() {
    input! {
        (N, M): (usize, usize),
        adb: [(usize, usize, usize); N],
    }

    let mut add: Vec<HashMap<usize, usize>> = vec![HashMap::new(); M];
    let mut sub: Vec<HashMap<usize, usize>> = vec![HashMap::new(); M];

    for &(a, d, b) in adb.iter() {
        *add[d - 1].entry(b).or_default() += 1;
        *sub[d - 1].entry(a).or_default() += 1;
    }

    let mut color: HashMap<usize, usize> = HashMap::new();

    for &(a, _, _) in adb.iter() {
        *color.entry(a).or_default() += 1;
    }

    for day in 0..M {
        for (&add_color, &add_cnt) in add[day].iter() {
            *color.entry(add_color).or_default() += add_cnt;
        }

        for (&sub_color, &sub_cnt) in sub[day].iter() {
            *color.entry(sub_color).or_default() -= sub_cnt;
            if color[&sub_color] == 0 {
                color.remove(&sub_color);
            }
        }

        // eprintln!("{:?}", color);
        println!("{}", color.len());
    }
}
