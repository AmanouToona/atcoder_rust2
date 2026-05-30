use itertools::Itertools;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut y_max = N.isqrt();
    if y_max * y_max < N {
        y_max += 1;
    }

    let mut cnt = vec![0; N + 1];
    for x in 1..=y_max {
        for y in x + 1..=y_max {
            if x * x + y * y > N {
                break;
            } else {
                cnt[x * x + y * y] += 1;
            }
        }
    }

    let ans: Vec<usize> = cnt
        .iter()
        .enumerate()
        .skip(1)
        .filter(|(_, c)| c == &&1)
        .map(|x| x.0)
        .collect();

    println!("{}", ans.len());
    let ans: String = ans.iter().join(" ");
    println!("{ans}");
}
