use itertools::Itertools;
use proconio::input;

fn dfs(
    u: usize,
    ans: &mut Vec<usize>,
    g: &Vec<Vec<(usize, usize)>>,
    cost: usize,
    S: usize,
    T: usize,
    L: usize,
    cnt: usize,
) {
    if cnt == L {
        if cost >= S && cost <= T {
            ans.push(u);
        }
        return;
    }

    for &(v, c) in g[u].iter() {
        dfs(v, ans, g, cost + c, S, T, L, cnt + 1);
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M, L, S, T):( usize, usize, usize, usize, usize),
        UVC: [(usize, usize, usize); M],
    }

    let mut g = vec![Vec::new(); N];

    for &(u, v, c) in UVC.iter() {
        let u = u - 1;
        let v = v - 1;
        g[u].push((v, c));
    }

    let mut ans = Vec::new();
    dfs(0, &mut ans, &g, 0, S, T, L, 0);

    ans.sort();
    ans.dedup();
    let ans : String = ans.iter().map(|x| x + 1).join(" ");

    println!("{ans}");
}
