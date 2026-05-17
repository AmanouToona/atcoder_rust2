#![allow(non_snake_case)]
use ac_library::ModInt998244353 as mint;
use proconio::input;
/*
遅延セグ木によるDPが可能もっと楽な方法がある？

自分の前の数字は自分以下の数字であることが確定しているので、状態数の累積和で更新できる

*/

fn main() {
    input! {
        N: usize,
        a: [usize; N],
        b: [usize; N],
    }

    let mut c = vec![mint::new(0); 3001];
    c[0] = mint::new(1);

    for (&a, &b) in a.iter().zip(b.iter()) {
        let mut nxt = vec![mint::new(0); 3001];
        let mut cumsum: mint = c.iter().take(a).sum();

        for i in a..=b {
            cumsum += c[i];
            nxt[i] = cumsum;
        }
        c = nxt;
    }

    let ans: mint = c.iter().sum();
    println!("{ans}");
}
