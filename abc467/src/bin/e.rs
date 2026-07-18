#![allow(non_snake_case)]
use proconio::input;
/*
M <= 10 ** 9 が難しい。 状態を持てない
なぜ M == 2　は簡単なのか? ... modMの状態が少ないから ... 状態を modM で持つのは無理 ... 最後の値をみて DP は無理

若干のグラフっぽさを感じる
ジャンプする点がある。 それはどこ?
... 非連続面がある問題 なにかでやった気がする
もしかして、どこか一箇所は0回で固定なのでは？

*/

fn main() {
    input! {
        (N, M): (usize, usize),
        A: [usize; N],
        B: [usize; N  - 1]
    }

    let mut ans = usize::MAX;
    for i in 0..N {
        // ai は固定する
        let mut prea = A[i];
        let mut tmp = 0;
        for (j, &b) in B.iter().enumerate().cycle().skip(i).take(N - 1) {
            let i = (j + 1) % N;
            if prea + A[i] <= b {
                tmp += b - (prea + A[i]);
                prea = b - prea;
            } else {
                let mut b2 = b;
                while b2 < prea + A[i] {
                    b2 += M;
                }
                tmp += b2 - (prea + A[i]);
                prea = b2 - prea;
            }
            eprintln!("{j} {b} {i}");
        }
        eprintln!("{tmp}");
        ans = ans.min(tmp);
        eprintln!("");
    }
    println!("{ans}");
}
