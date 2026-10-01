#![allow(non_snake_case)]
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::mem;

/*
0 index
- N が特殊な頂点 全点に連結
- 複数回Nを通ることはない. N は1回.
*/
fn main() {
    input! {
        (N, Q): (usize, usize),
        A: [usize; N],
        B: [usize; N],
        st: [(usize, usize); Q],
    }

    // 累積和用意
    let mut cumsum = vec![0];
    cumsum.extend(A.iter().cycle().take(N * 2));
    for i in 0..cumsum.len() - 1 {
        cumsum[i + 1] += cumsum[i];
    }

    // 頂点Nからの最小距離
    let mut fromN = vec![usize::MAX; N + 1];
    fromN[N] = 0;

    let mut heap: BinaryHeap<(Reverse<usize>, usize)> = BinaryHeap::new();
    for (i, &b) in B.iter().enumerate() {
        heap.push((Reverse(b), i));
    }
    while let Some((Reverse(b), u)) = heap.pop() {
        if fromN[u] != usize::MAX {
            continue;
        }

        fromN[u] = b;
        let v = (u + 1) % N;
        if fromN[v] == usize::MAX {
            heap.push((Reverse(b + A[u]), v));
        }

        let v = (u + N - 1) % N;
        if fromN[v] == usize::MAX {
            heap.push((Reverse(b + A[v]), v));
        }
    }

    // 回答
    for &(s, t) in st.iter() {
        let mut s = s - 1;
        let mut t = t - 1;

        if t < s {
            mem::swap(&mut s, &mut t);
        }

        // N + 1 を通る
        let mut ans = fromN[s] + fromN[t];

        // N + 1 を通らない
        if s != N && t != N {
            ans = ans
                .min(cumsum[t] - cumsum[s])
                .min(cumsum[s + N] - cumsum[t]);
        }
        println!("{ans}");
    }
}
