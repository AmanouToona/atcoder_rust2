use proconio::input;
use std::collections::HashMap;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut count: HashMap<usize, usize> = HashMap::new();

    for &a in A.iter() {
        *count.entry(a).or_default() += 1;
    }

    let count: Vec<usize> = count.into_values().collect();
    let combination: Vec<usize> = count.iter().map(|&x| x * (x - 1) / 2).collect();
    let combi_sum: usize = combination.iter().sum();

    let mut ans = 0;
    for (c, d) in count.iter().zip(combination.iter()) {
        ans += c * (combi_sum - d);
    }

    println!("{ans}");
}
