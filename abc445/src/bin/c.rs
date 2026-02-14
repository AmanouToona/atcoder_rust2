use itertools::Itertools;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut dp = vec![vec![0; N]; 40];
    for (i, &a) in A.iter().enumerate() {
        dp[0][i] = a - 1;
    }

    for i in 1..40 {
        for j in 0..N {
            dp[i][j] = dp[i - 1][dp[i - 1][j]];
        }
    }

    let mut ans = Vec::new();
    for i in 0..N {
        let mut state = i;
        for _ in 0..10 {
            let mut rest: usize = 10_i64.pow(10) as usize;
            for j in 0..40 {
                if rest >> j & 1 == 1 {
                    state = dp[j][state];
                }
            }
        }
        ans.push(state);
    }

    let ans: String = ans.iter().map(|x| *x + 1).join(" ");
    println!("{ans}");
}
