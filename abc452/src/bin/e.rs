#![allow(non_snake_case)]
use ac_library::ModInt998244353 as Mint;
use proconio::input;
fn main() {
    input! {
        (N, M): (usize, usize),
        A: [Mint; N],
        B: [Mint; M],
    }

    let mut ans = Mint::new(0);

    let mut ai_sum = Mint::new(0);
    for (i, &a) in A.iter().enumerate() {
        ai_sum += Mint::new(i + 1) * a;
    }

    for &b in B.iter() {
        ans += b * ai_sum;
    }

    let mut cumsum = vec![Mint::new(0); N + 2];
    for (i, a) in A.iter().enumerate() {
        cumsum[i + 2] = *a;
        let (i1, i2) = cumsum.split_at_mut(i + 2);
        i2[0] += i1.last().unwrap();
    }

    for (j, b) in B.iter().enumerate() {
        let j = j + 1; // 式を問題文に合わせるため
        let mut aij = Mint::new(0);
        let mut i = 1;
        while i <= N {
            let nxt = (N + 1).min((i / j + 1) * j);
            aij += (cumsum[nxt] - cumsum[i]) * Mint::new(i / j) * Mint::new(j);
            i = nxt;
        }
        ans -= b * aij;
    }

    println!("{ans}");
}
