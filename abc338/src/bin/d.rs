#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, M): (usize, usize),
        X: [usize; M],
    }

    let mut cost = vec![0i128; N + 2];
    let mut min_cost = 0;
    for (&ai, &aj) in X.iter().zip(X.iter().skip(1)) {
        let (ai, aj) = if ai < aj { (ai, aj) } else { (aj, ai) };

        let min_diff = (ai + N - aj).min(aj - ai);
        let max_diff = N - min_diff;

        min_cost += min_diff;

        let diff = (max_diff - min_diff) as i128;
        if diff == 0 {
            continue;
        }

        if min_diff == aj - ai {
            cost[ai + 1] += diff;
            cost[aj + 1] -= diff;
        } else {
            cost[aj + 1] += diff;
            cost[1] += diff;
            cost[ai + 1] -= diff;
        }
    }

    for i in 0..cost.len() - 1 {
        cost[i + 1] += cost[i];
    }

    let mut ans = usize::MAX;
    for i in 1..=N {
        ans = ans.min(min_cost + cost[i] as usize);
    }

    println!("{ans}");
}
