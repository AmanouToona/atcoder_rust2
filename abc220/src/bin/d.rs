#![allow(non_snake_case)]
use ac_library::ModInt998244353 as mint;
use proconio::input;
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut num = vec![mint::new(0); 10];
    num[A[0]] = mint::new(1);
    for &a in A.iter().skip(1) {
        let pre = num.clone();
        num = vec![mint::new(0); 10];
        for i in 0..10 {
            // f
            let nxt = (a + i) % 10;
            num[nxt] += pre[i];

            // g
            let nxt = (a * i) % 10;
            num[nxt] += pre[i];
        }
    }

    for i in num {
        println!("{i}");
    }
}
