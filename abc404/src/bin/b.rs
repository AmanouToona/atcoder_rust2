#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        N: usize,
        mut S: [Chars; N],
        T: [Chars; N],
    }

    let mut ans = usize::MAX;
    for i in 0..4 {
        let mut now = i;
        for i in 0..N {
            for j in 0..N {
                if S[i][j] != T[i][j] {
                    now += 1;
                }
            }
        }

        ans = ans.min(now);

        // 更新
        let mut new: Vec<Vec<char>> = vec![vec!['.'; N]; N];
        for j in 0..N {
            for k in 0..N {
                new[k][N - 1 - j] = S[j][k];
            }
        }
        S = new;
    }

    println!("{ans}");
}
