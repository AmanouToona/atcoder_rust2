use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut AB: [(usize, usize); N]
    }

    AB.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));

    let seq: Vec<usize> = AB.iter().map(|x| x.1).collect();

    // LIS
    let mut dp = Vec::new();
    for b in seq.iter() {
        let pos = dp.binary_search(b).unwrap_or_else(|x| x);
        if pos < dp.len() {
            dp[pos] = *b;
        } else {
            dp.push(*b);
        }
    }

    println!("{}", dp.len());
}
