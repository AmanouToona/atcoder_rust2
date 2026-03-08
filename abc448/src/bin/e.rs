#![allow(non_snake_case)]
use ac_library::ModInt as Mint;
use proconio::input;
fn main() {
    input! {
        (K, M): (usize, usize),
        cl :[(usize, usize); K],
    }

    let modulo = 10007;
    Mint::set_modulus((M * modulo) as u32);

    //  pow[k] = 10 ** (k + 1);
    let mut pow = [Mint::new(0); 30];
    pow[0] = Mint::new(10);
    for i in 0..pow.len() - 1 {
        pow[i + 1] = pow[i] * pow[i];
    }

    // ones[k] = 1 k digit;
    let mut ones = [Mint::new(1); 30];
    for i in 1..ones.len() {
        ones[i] = ones[i - 1] * pow[i - 1] + ones[i - 1];
    }

    // N = MBQ + X
    let mut x = Mint::new(0);
    for &(c, l) in cl.iter() {
        let mut digit = Mint::new(1);
        let mut one = Mint::new(0);
        for (i, (&p, &o)) in pow.iter().zip(ones.iter()).enumerate() {
            if l >> i & 1 == 1 {
                digit *= p;
                one = one * p + o;
            }
        }
        x = x * digit + one * Mint::new(c);
    }

    let ans = (x.val() as usize / M) % modulo;
    println!("{ans}");
}
