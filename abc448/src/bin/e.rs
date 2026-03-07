#![allow(non_snake_case)]
use ac_library::ModInt as Mint;
use proconio::input;
fn main() {
    input! {
        (K, M): (usize, usize),
        cl :[(usize, usize); K],
    }

    //　完全に間違っている 発想が違う
    let mo: usize = 10007;
    Mint::set_modulus(mo as u32);

    let mut doubling = [Mint::new(0); 30];
    doubling[0] = Mint::new(10);
    for i in 1..doubling.len() {
        doubling[i] = doubling[i - 1] * doubling[i - 1];
    }

    let mut now = Mint::new(0);

    for &(c, l) in cl.iter() {
        let mut digit = Mint::new(1);
        for d in 0..=30 {
            if (l >> d) & 1 == 1 {
                digit *= doubling[d];
            }
        }
        now *= digit;
        now += Mint::new(c) * (digit - Mint::new(1)) / Mint::new(9);
        eprintln!("{now}");
    }

    let n = now.val() as usize * mo * mo;
    eprintln!("n {n}");
    let ans = (n / M) % mo;
    println!("{ans}");
}
