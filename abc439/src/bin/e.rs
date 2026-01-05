use proconio::input;
use std::collections::HashMap;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut AB: [(usize, usize); N]
    }

    let mut B: Vec<usize> = AB.iter().map(|x| x.1).collect();
    B.sort();

    let mut map: HashMap<usize, usize> = HashMap::new();
    for &b in B.iter() {
        let len = map.len();
        let _ = *map.entry(b).or_insert(len);
    }

    AB.sort_by(|x, y| y.1.cmp(&x.1));
    AB.sort_by(|x, y| x.0.cmp(&y.0));

    let new_b: Vec<usize> = AB.iter().map(|x| *map.get(&x.1).unwrap()).collect();

    let mut dp = vec![usize::MAX];
    for &b in new_b.iter() {
        if dp[0] > b {
            dp[0] = b;
            continue;
        }

        if dp.last().unwrap() < &b {
            dp.push(b);
            continue;
        }

        if dp.last().unwrap() == &b {
            continue;
        }

        let mut left = 0;
        let mut right = dp.len();

        while right - left > 1 {
            let mid = (right + left) / 2;

            if dp[mid] < b {
                left = mid;
            } else {
                right = mid;
            }
        }
        dp[right] = b;
    }

    println!("{}", dp.len());
}
