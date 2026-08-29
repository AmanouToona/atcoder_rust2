#![allow(non_snake_case)]
use ac_library::Dsu;
use proconio::input;
use std::collections::HashMap;
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let a_max = *A.iter().max().unwrap();
    let mut is_in = vec![false; a_max + 1];
    let mut a_to_no: HashMap<usize, usize> = HashMap::new();
    for &a in A.iter() {
        is_in[a] = true;
        a_to_no.insert(a, a_to_no.len());
    }

    let mut ans = 0;
    let mut union_find = Dsu::new(N);
    for i in (1..=a_max).rev() {
        let mut candidate = Vec::new();

        for j in (i..=a_max).step_by(i) {
            if is_in[j] {
                candidate.push(j);
            }
        }

        if candidate.len() <= 1 {
            continue;
        }

        for &a in candidate.iter().skip(1) {
            if !union_find.same(a_to_no[&a], a_to_no[&candidate[0]]) {
                union_find.merge(a_to_no[&a], a_to_no[&candidate[0]]);
                ans += i;
            }
        }
    }
    println!("{ans}");
}
