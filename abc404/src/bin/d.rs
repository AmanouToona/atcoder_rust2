#![allow(non_snake_case)]
use proconio::input;

fn dfs(A: &[Vec<usize>], C: &[usize], u: usize, sum: usize, seen: &mut [usize]) -> usize {
    // ブロック
    if seen.iter().all(|x| *x >= 2) {
        return sum;
    }
    if u == A.len() {
        return usize::MAX;
    }

    // 更新
    let mut res = usize::MAX;
    for count in 0..3 {
        for &animal in A[u].iter() {
            seen[animal] += count;
        }
        res = res.min(dfs(A, C, u + 1, sum + C[u] * count, seen));

        for &animal in A[u].iter() {
            seen[animal] -= count;
        }
    }

    res
}

fn main() {
    input! {
        (N, M): (usize, usize),
        C: [usize; N],
    }

    let mut A: Vec<Vec<usize>> = vec![Vec::new(); N];
    for i in 0..M {
        input! {K: usize, a:[usize; K]}
        for &a in a.iter() {
            A[a - 1].push(i);
        }
    }

    // 状態は 3 ** 10 しかない
    let ans = dfs(&A, &C, 0, 0, &mut vec![0; M]);
    println!("{ans}");
}
