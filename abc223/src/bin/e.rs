#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;

/*
特徴
- 必ずある長方形の頂点を原点に触れさせることが可能

- dfs?
- ある長方形を x, y に張り付かせる？
*/

fn can(x: usize, y: usize, a: usize, b: usize, c: usize) -> bool {
    if a.div_ceil(x) >= y {
        return false;
    }

    let res_y = y - a.div_ceil(x);

    // x, res_y の四角形が使える
    let want_x = b.div_ceil(res_y) + c.div_ceil(res_y);
    if want_x <= x {
        return true;
    }

    let want_y = b.div_ceil(x) + c.div_ceil(x);
    want_y <= res_y
}

fn main() {
    input! {
        (x, y, a, b, c): (usize, usize, usize, usize, usize),
    }

    for i in [a, b, c].iter().permutations(3) {
        if can(x, y, *i[0], *i[1], *i[2]) {
            println!("Yes");
            return;
        }
        if can(y, x, *i[0], *i[1], *i[2]) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
