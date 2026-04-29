#![allow(non_snake_case)]
use ac_library::ModInt998244353 as mint;
use amplify::confinement::Collection;
use num::Integer;
use proconio::input;
use std::collections::HashMap;
use std::collections::VecDeque;
/*
2 ** N は実行不能なので、制約を見つけよう
ある箇所を決めたら自動的に決定される箇所はあるか？

探索回数について、 u -> v の探索は O(2N) M 回繰り返しても O(NM) なので探索可能
それぞれの辺の通過回数は算出できる

通過回数一覧から、 K を作成する方法は何通りか？
K を状態にもつ DP でいける?
dp[i][k] := i 個目の辺まで確認して、R-B=K が k になる状態の個数
通過回数の最大値は、M - 1 なので、dpの中の k は (M - 1)(N - 1) の範囲で移動し得る. 問題で与えられるKの方がきつい制約になる
計算量は、 Nk になるが、 k をどこまで取る必要があるだろうか？ ... 10**6 になりそうだからこれでは回らない？

R - B = K
R + B = S ... 移動距離の総和
-> 2R = K + S
S は、 0 ~ (M - 1)(N - 1)
K = -10**5..=10**5
K + S = 0..(M - 1)(N - 1) + 10**5  ... K + S < 0 なら解なし
おおよそ O(10**5) のオーダー
これをエッジについて回すので全体計算量は O(10**8)
*/

fn main() {
    input! {
        (N, M, K): (usize, usize, i64),
        A: [usize; M],
        uv: [(usize, usize); N - 1]
    }

    let mut g = vec![Vec::new(); N];
    for &(u, v) in uv.iter() {
        let u = u - 1;
        let v = v - 1;
        g[u].push(v);
        g[v].push(u);
    }

    let mut edge_cnt: HashMap<(usize, usize), usize> = HashMap::new();
    for (&start, &goal) in A.iter().zip(A.iter().skip(1)) {
        let start = start - 1;
        let goal = goal - 1;
        let mut q = VecDeque::new();
        q.push(start);
        let mut from: Vec<Option<usize>> = vec![None; N];
        from[start] = Some(start);

        'bfs: while let Some(u) = q.pop_front() {
            for &v in g[u].iter() {
                if from[v].is_none() {
                    from[v] = Some(u);
                    q.push_back(v);
                    if v == goal {
                        break 'bfs;
                    }
                }
            }
        }

        let mut v = goal;
        while v != start {
            let u = from[v].unwrap();
            if v < u {
                *edge_cnt.entry((v, u)).or_default() += 1;
            } else {
                *edge_cnt.entry((u, v)).or_default() += 1;
            }
            v = u;
        }
    }

    // dp[i] := R を i 個にする組み合わせの個数
    let mut dp = vec![mint::new(0); 200_001];
    dp[0] = mint::new(1);
    for &v in edge_cnt.values() {
        let mut next = dp.clone();
        for i in 0..=200_000 {
            let j = i + v;
            if j < next.len() {
                next[j] += dp[i];
            }
        }
        dp = next;
    }

    let s: usize = edge_cnt.values().cloned().sum::<usize>();
    if K + (s as i64) < 0 {
        println!("0");
        return;
    }
    let sum_ks = (K + s as i64) as usize;
    if sum_ks.is_odd() {
        println!("0");
        return;
    }

    let mut ans = dp[sum_ks / 2];
    // 何色で塗っても良いものを処理
    for _ in 0..N - 1 - edge_cnt.len() {
        ans *= mint::new(2);
    }
    println!("{ans}");
}
