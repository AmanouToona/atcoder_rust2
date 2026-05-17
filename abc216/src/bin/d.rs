#![allow(non_snake_case)]
use amplify::confinement::Collection;
use proconio::input;
use std::collections::VecDeque;

/*
管理する必要がある情報は
- 筒のトップの色
- トップの色がどの筒にあるか？
- 削除可能な筒はどれか？

->
- トップの色の位置
- 削除可能な筒
*/

fn main() {
    input! {
        (N, M): (usize, usize),
    }

    let mut color2pos: Vec<Vec<usize>> = vec![Vec::new(); N];
    let mut pos2del: VecDeque<(usize, usize)> = VecDeque::new();
    let mut A: Vec<Vec<usize>> = Vec::new();

    for m in 0..M {
        input! {
            k: usize,
            a: [usize; k]
        }
        let a: Vec<usize> = a.into_iter().map(|x| x - 1).collect();
        let top_color = *a.last().unwrap();
        A.push(a);
        color2pos[top_color].push(m);
        if color2pos[top_color].len() == 2 {
            pos2del.push((color2pos[top_color][0], color2pos[top_color][1]));
        }
    }

    while let Some((pos1, pos2)) = pos2del.pop_front() {
        A[pos1].pop();
        if let Some(&nxt) = A[pos1].last() {
            color2pos[nxt].push(pos1);
            if color2pos[nxt].len() == 2 {
                pos2del.push_back((color2pos[nxt][0], color2pos[nxt][1]));
            }
        }

        A[pos2].pop();
        if let Some(&nxt) = A[pos2].last() {
            color2pos[nxt].push(pos2);
            if color2pos[nxt].len() == 2 {
                pos2del.push_back((color2pos[nxt][0], color2pos[nxt][1]));
            }
        }
    }

    for a in A.iter() {
        if !a.is_empty() {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
