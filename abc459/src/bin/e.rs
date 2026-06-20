#![allow(non_snake_case)]
use ac_library::ModInt998244353 as mint;
use proconio::input;
/*
葉に近い方のリスから処理
残りの飴玉について選択肢が生じる

dfs で葉から処理
*/

struct Solve {
    g: Vec<Vec<usize>>,
    c: Vec<usize>,
    d: Vec<usize>,
    ifrac: Vec<mint>,
}

impl Solve {
    fn new(P: &[usize], C: &[usize], D: &[usize]) -> Self {
        let N = P.len() + 1;
        let mut g = vec![Vec::new(); N];
        for (i, &p) in P.iter().enumerate() {
            g[p - 1].push(i + 1);
        }

        let mut frac = mint::new(1);
        for i in 1..=10usize.pow(6) {
            frac *= mint::new(i);
        }

        let mut ifrac = vec![mint::new(1); 10usize.pow(6) + 1];
        ifrac[10usize.pow(6)] = mint::new(1) / frac;
        for i in (1..=10usize.pow(6)).rev() {
            ifrac[i - 1] = ifrac[i] * mint::new(i);
        }

        Solve {
            g,
            c: C.to_vec(),
            d: D.to_vec(),
            ifrac,
        }
    }

    fn dfs(&self, u: usize, cmb: &mut Vec<mint>) -> Option<usize> {
        let mut can_take = self.c[u];
        for &v in self.g[u].iter() {
            can_take += self.dfs(v, cmb)?
        }

        if can_take < self.d[u] {
            None
        } else {
            cmb[u] = self.combination(can_take, self.d[u]);
            Some(can_take - self.d[u])
        }
    }

    fn combination(&self, n: usize, m: usize) -> mint {
        let mut res = mint::new(1);
        for i in (0..=n).rev().take(m) {
            res *= mint::new(i);
        }

        res * self.ifrac[m]
    }
}

fn main() {
    input! {
        N: usize,
        P: [usize; N - 1],
        C: [usize; N],
        D: [usize; N],
    }

    let solve = Solve::new(&P, &C, &D);

    let mut combination = vec![mint::new(0); N];
    match solve.dfs(0, &mut combination) {
        Some(_) => {
            let ans: mint = combination.iter().product();
            println!("{ans}");
        }
        None => {
            println!("0")
        }
    }
}
