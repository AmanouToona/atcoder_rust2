#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
use std::collections::BTreeMap;
fn main() {
    input! {
        Q: usize
    }

    let mut cnt: Vec<usize> = vec![0]; // ノードに存在する y の個数
    let mut to: Vec<BTreeMap<char, usize>> = vec![BTreeMap::new()];
    let mut from: Vec<usize> = vec![usize::MAX]; // usize::MAX is sentinel
    let mut is_end: Vec<bool> = vec![false];

    for _ in 0..Q {
        input! {
            T: usize,
            S: Chars,
        }

        let mut node_num: usize = 0;
        let mut do_revert = false;
        if T == 2 {
            cnt[0] += 1;
        }
        for (i, &s) in S.iter().enumerate() {
            let len = cnt.len();
            if !to[node_num].contains_key(&s) {
                to[node_num].insert(s, len);
                to.push(BTreeMap::new());
                cnt.push(0);
                from.push(node_num);
                is_end.push(false);
            }

            node_num = to[node_num].get(&s).unwrap().clone();
            if T == 2 {
                cnt[node_num] += 1;
            } else if i == S.len() - 1 {
                is_end[node_num] = true;
            }

            if is_end[node_num] {
                do_revert = true;
                break;
            }
        }
        if do_revert {
            let sub = cnt[node_num];
            while node_num != usize::MAX {
                cnt[node_num] -= sub;
                node_num = from[node_num];
            }
        }
        let ans = cnt[0];
        println!("{ans}");
    }
}
