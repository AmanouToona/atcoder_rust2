#![allow(non_snake_case)]
use proconio::input;
use std::collections::BTreeMap;
fn main() {
    input! {
        Q: usize
    }

    let mut tot = 0;
    let mut tree: BTreeMap<usize, usize> = BTreeMap::new();

    for _ in 0..Q {
        input! {q: usize, h: usize}
        match q {
            1 => {
                *tree.entry(h).or_default() += 1;
                tot += 1;
            }
            2 => {
                let mut to_remove = Vec::new();
                for (k, v) in tree.range(..=h) {
                    tot -= *v;
                    to_remove.push(*k);
                }

                for r in to_remove.iter() {
                    tree.remove(r);
                }
            }
            _ => {
                panic!()
            }
        }

        println!("{tot}");
    }
}
