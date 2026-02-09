use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        WHB: [(usize, i64, i64); N],
    }

    let max_weight = 500 * 500;
    let mut dp: Vec<i64> = vec![0; max_weight + 1];
    let mut used = vec![false; max_weight + 1];
    used[0] = true;

    for &(w, h, b) in WHB.iter() {
        for uw in (0..=max_weight).rev() {
            if !used[uw] {
                continue;
            }
            let vw = uw + w;

            if uw != 0 && dp[uw] == 0 {
                continue;
            }

            let d_value = h - b;
            dp[vw] = dp[vw].max(dp[uw] + d_value);
            used[vw] = true;
        }
    }

    let sum_w: usize = WHB.iter().map(|x| x.0).sum();

    let ans: i64 =
        WHB.iter().map(|x| x.2).sum::<i64>() + dp.iter().take(sum_w / 2 + 1).max().unwrap_or(&0);

    println!("{ans}");
}
