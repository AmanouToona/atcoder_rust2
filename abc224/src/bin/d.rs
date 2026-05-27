#![allow(non_snake_case)]
use proconio::input;
use std::collections::HashMap;
use std::collections::VecDeque;

/*
とりうる状態の数は
9 * 8 * ... * 1 = 362880 ... 4 * 10 ** 5
*/
fn main() {
    input! {
        M: usize,
        uv: [(usize, usize); M],
        P: [usize; 8]
    }

    let mut g = vec![Vec::new(); 9];
    for &(u, v) in uv.iter() {
        g[u - 1].push(v - 1);
        g[v - 1].push(u - 1);
    }

    let mut seen: HashMap<[usize; 9], usize> = HashMap::new();
    let mut q = VecDeque::new();

    let mut state = [8; 9]; // 頂点iに置かれている駒
    for (i, p) in P.iter().enumerate() {
        state[p - 1] = i;
    }
    q.push_back(state);
    seen.insert(state, 0);

    while let Some(state_u) = q.pop_front() {
        if state_u == [0, 1, 2, 3, 4, 5, 6, 7, 8] {
            println!("{}", seen.get(&state_u).unwrap());
            return;
        }
        let u = state_u.iter().position(|x| *x == 8).unwrap();

        let mut state_v = state_u;
        for &v in g[u].iter() {
            state_v.swap(u, v);

            if !seen.contains_key(&state_v) {
                q.push_back(state_v);
                seen.insert(state_v, seen.get(&state_u).unwrap() + 1);
            }
            state_v.swap(u, v);
        }
    }
    println!("-1");
}
