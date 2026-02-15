use ac_library::ModInt998244353 as mint;
use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;

#[allow(non_snake_case)]
fn make_osa(n: usize) -> Vec<usize> {
    let mut osa: Vec<usize> = (0..=n).collect();
    for i in 2..=n {
        if osa[i] != i {
            continue;
        }

        for j in (i * i..=n).step_by(i) {
            if osa[j] == j {
                osa[j] = i;
            }
        }
    }

    osa
}

#[allow(non_snake_case)]
fn main() {
    input! {T: usize}
    let osa = make_osa(10_000_000);
    for _ in 0..T {
        input! {
            N: usize,
            A: [usize; N],
        }

        let mut max_exponent: HashMap<usize, (usize, usize)> = HashMap::new();
        let mut factors = Vec::new();
        for &a in A.iter() {
            let mut tmp = a;
            let mut factor = Vec::new();
            while tmp > 1 {
                let mut exp = 0;
                let p = osa[tmp];
                while tmp % p == 0 {
                    tmp /= p;
                    exp += 1;
                }
                factor.push((p, exp));
                max_exponent
                    .entry(p)
                    .and_modify(|(max1, max2)| {
                        if *max1 < exp {
                            *max2 = *max1;
                            *max1 = exp;
                        } else if *max2 < exp {
                            *max2 = exp;
                        }
                    })
                    .or_insert((exp, 0));
            }
            factors.push(factor);
        }

        let total_lcm: mint = max_exponent
            .iter()
            .map(|(&k, &(max1, _))| mint::new(k).pow(max1 as u64))
            .product();

        let mut ans = Vec::new();
        for factor in factors.iter() {
            let mut sub = mint::new(1);

            for &(p, exp) in factor.iter() {
                let &(max1, max2) = max_exponent.get(&p).unwrap();
                if exp == max1 {
                    sub *= mint::new(p).pow((max1 - max2) as u64);
                }
            }
            ans.push(total_lcm / sub);
        }

        let ans: String = ans.iter().join(" ");
        println!("{ans}");
    }
}
