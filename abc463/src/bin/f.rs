#![allow(non_snake_case)]
use ac_library::ModInt998244353 as mint;
use proconio::input;

/*
優勝できる人は、今の勝利数が、
- 最大の人
- 最大 - 1 の人
だけ
なので、優勝者の勝利数が、a.max + 1, a.max だけを気にすれば良い

処理が別に必要
- max vs max - 1
- max vs max
- max - 1 vs max - 1


*/
fn main() {
    input! {
        N: usize,
        A: [(usize, usize ) ; N],
    }

    let a_max = A.iter().map(|x| x.0.max(x.1)).max().unwrap();
    let A: Vec<(usize, usize)> = A
        .into_iter()
        .filter(|&x| x.0 + 1 >= a_max || x.1 + 1 >= a_max)
        .collect();
    eprintln!("{:?}", A);

    // let A = A.iter().filter(|&(x, y)

    // let a_max = *A.iter().max().unwrap();

    // // a_max = 0 が例外

    // //
    // let n = A.iter().filter(|&a| *a == a_max).count();
    // let m = A.iter().filter(|&a| *a == a_max - 1).count();

    // let mut frac = vec![mint::new(1); n.max(m) + 1];
    // for i in 1..n.max(m) + 1 {
    //     frac[i] = frac[i - 1] * mint::new(i);
    // }

    // let mut ifrac = vec![mint::new(1); n.max(m) + 1];
    // ifrac[n.max(m)] = mint::new(1) / frac[n.max(m)];
    // for i in (0..n.max(m)).rev() {
    //     ifrac[i] = ifrac[i + 1] * mint::new(i + 1);
    // }

    // eprintln!("{} {}", n, m);

    // let mut two_n = mint::new(1);
    // for _ in 0..n {
    //     two_n /= mint::new(2);
    // }

    // let mut two_m = mint::new(1);
    // for _ in 0..m {
    //     two_m /= mint::new(2);
    // }

    // let mut win_1 = mint::new(0); // 優勝数 max -> max + 1 で優勝する確率
    // for i in 0..n {
    //     win_1 += two_n / mint::new(i + 1);
    // }

    // let mut win_2 = mint::new(0); // max -> max で優勝する確率
    // for i in 0..=m {
    //     win_2 += two_n * two_m / mint::new(i + n);
    // }

    // let mut win_3 = mint::new(0); // max - 1 -> max で優勝する確率
    // for i in 1..=m {
    //     win_3 +=
    //         two_n * two_m / mint::new(n + i) * frac[m - 1] * ifrac[i - 1] * ifrac[m - 1 - (i - 1)];
    // }

    // eprintln!("{} {} {}", win_1, win_2, win_3);
    // eprintln!("{} {}", win_1 + win_2, win_3);
}
