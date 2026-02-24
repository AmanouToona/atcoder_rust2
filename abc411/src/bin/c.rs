#![allow(non_snake_case)]
use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {
        (_, Q): (usize, usize),
        A: [usize; Q],
    }

    let mut black = HashSet::new();
    let mut ans = 0;
    for &a in A.iter() {
        if black.contains(&a) {
            // 白色に変える
            if black.contains(&(a - 1)) & black.contains(&(a + 1)) {
                ans += 1;
            } else if !black.contains(&(a - 1)) & !black.contains(&(a + 1)) {
                ans -= 1;
            }

            // 更新
            black.remove(&a);
        } else {
            // 黒色に変える
            if black.contains(&(a - 1)) & black.contains(&(a + 1)) {
                ans -= 1;
            } else if !black.contains(&(a - 1)) & !black.contains(&(a + 1)) {
                ans += 1;
            }

            // 更新
            black.insert(a);
        }
        println!("{ans}");
    }
}
