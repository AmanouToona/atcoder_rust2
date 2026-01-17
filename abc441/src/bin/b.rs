use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        S: Chars,
        T: Chars,
        Q: usize,
    }

    let mut t_set: HashSet<char> = HashSet::new();
    for s in S.iter() {
        t_set.insert(*s);
    }

    let mut a_set: HashSet<char> = HashSet::new();
    for t in T.iter() {
        a_set.insert(*t);
    }

    let ans_t = "Takahashi";
    let ans_a = "Aoki";
    let ans_u = "Unknown";

    for _ in 0..Q {
        input! {
            w: Chars,
        }

        let mut is_t = true;
        let mut is_a = true;

        for ww in w.iter() {
            if !t_set.contains(ww) {
                is_t = false;
            }
            if !a_set.contains(ww) {
                is_a = false;
            }
        }

        if is_t && !is_a {
            println!("{ans_t}");
        } else if !is_t && is_a {
            println!("{ans_a}");
        } else {
            println!("{ans_u}");
        }
    }
}
