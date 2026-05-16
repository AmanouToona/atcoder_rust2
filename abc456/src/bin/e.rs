#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;

/*
同じ都市に同じ曜日に戻ってくることができるか？　を確認する問題
都市の数は 10**5 曜日の数は 10 なので　都市と曜日で状態は 10**6 これをノードとして考えることができる
同じノードに戻って来れるか？　という問題にできる

この時、頂点の数は 10**6 で、辺の数は、 10**6 (...雑に)
計算量は O(ノード数 + エッジ数) なので、充分に間に合う

...
同じノードに戻るの判断。
すでに見た　を元に判断すると違うルートを通っているものと区別ができない。

グラフがループを持つかを確認する問題?
そうだ　ループ問題に言い換えることができる。
向こうグラとして与えられたものを、曜日の状態を含む有向グラフに変換しているから

*/

fn dfs(u: usize, g: &Vec<Vec<usize>>, seen: &mut Vec<bool>, finished: &mut Vec<bool>) -> bool {
    for &v in g[u].iter() {
        if finished[v] {
            return true;
        }

        if seen[v] {
            continue;
        }

        seen[v] = true;
        finished[v] = true;
        if dfs(v, g, seen, finished) {
            return true;
        };

        finished[v] = false;
    }
    false
}

fn main() {
    input! {
        T: usize
    }

    'outer: for _ in 0..T {
        input! {
            (N, M): (usize, usize),
            uv: [(usize, usize); M],
            W: usize,
            S: [Chars; N],
        }

        let mut g: Vec<Vec<usize>> = vec![Vec::new(); N * W];

        for &(u, v) in uv.iter() {
            let u = u - 1;
            let v = v - 1;

            for i in 0..W {
                let j = (i + 1) % W;
                if S[u][i] == 'o' && S[v][j] == 'o' {
                    g[u * W + i].push(v * W + j);
                }
                if S[v][i] == 'o' && S[u][j] == 'o' {
                    g[v * W + i].push(u * W + j);
                }
            }
        }

        for u in 0..N {
            for i in 0..W {
                let j = (i + 1) % W;
                if S[u][i] == 'o' && S[u][j] == 'o' {
                    g[u * W + i].push(u * W + j);
                }
            }
        }

        let mut seen = vec![false; N * W];
        let mut finised = vec![false; N * W];
        for s in 0..N {
            if dfs(s * W, &g, &mut seen, &mut finised) {
                println!("Yes");
                continue 'outer;
            }
        }

        println!("No");
    }
}
