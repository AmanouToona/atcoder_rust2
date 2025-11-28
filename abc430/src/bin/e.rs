use proconio::input;
use proconio::marker::Chars;
use rand::prelude::*;
use std::char;
struct RollingHash {
    hash: Vec<u64>,
    pow: Vec<u64>,
    modulo: u64,
}

impl RollingHash {
    fn new(s: &[char], base: u64, modulo: u64) -> Self {
        let n = s.len();
        let mut hash = vec![0; n + 1];
        let mut pow = vec![0; n + 1];
        pow[0] = 1;

        for i in 0..n {
            hash[i + 1] = (hash[i] * base + (s[i] as u64 + 1)) % modulo;
            pow[i + 1] = (pow[i] * base) % modulo;
        }
        RollingHash { hash, pow, modulo }
    }

    // [l, r)
    fn get(&self, l: usize, r: usize) -> u64 {
        (self.hash[r] + self.modulo - (self.hash[l] * self.pow[r - l]) % self.modulo) % self.modulo
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        T: usize
    }

    let mut rng = rand::thread_rng();

    'outer: for _ in 0..T {
        input! {A: Chars, B:Chars}

        let A2: Vec<char> = A.iter().cycle().take(A.len() * 2).cloned().collect();

        let rng1 = rng.gen_range(1..u32::MAX) as u64;
        let rng2 = rng.gen_range(1..u32::MAX) as u64;
        let rng3 = rng.gen_range(1..u32::MAX) as u64;

        let rha_1 = RollingHash::new(&A2, rng1, 998244353);
        let rha_2 = RollingHash::new(&A2, rng2, 1000000007);
        let rha_3 = RollingHash::new(&A2, rng3, 1000000009);

        let rhb_1 = RollingHash::new(&B, rng1, 998244353);
        let rhb_2 = RollingHash::new(&B, rng2, 1000000007);
        let rhb_3 = RollingHash::new(&B, rng3, 1000000009);

        for l in 0..A.len() {
            let r = l + A.len();
            if rha_1.get(l, r) == rhb_1.get(0, B.len())
                && rha_2.get(l, r) == rhb_2.get(0, B.len())
                && rha_3.get(l, r) == rhb_3.get(0, B.len())
            {
                println!("{}", l);
                continue 'outer;
            }
        }
        println!("-1");
    }
}
