#![allow(non_snake_case)]
use proconio::input;

/*
Ai の素因数が含まれないことが必要
最初に 1..=M の配列を作成して、条件を満たさないものを潰していくのが綺麗

潰し方は? Ai の素因数を先に全て判明させる？ 試し割は計算量の見積りが難しい
osa でやろう
*/

fn osa(n: usize) -> Vec<usize> {
    let mut res = vec![0; n + 1];

    for i in 2..=n {
        if res[i] != 0 {
            continue;
        }

        res[i] = i;
        let mut j = i * i;
        while j <= n {
            if res[j] == 0 {
                res[j] = i;
            }
            j += i;
        }
    }

    res
}

fn main() {
    input! {
        (N, M): (usize, usize),
        A: [usize; N]
    }

    let osa = osa(10usize.pow(5));
    let mut is_ans = vec![true; M + 1];

    let mut ps = std::collections::HashSet::new();
    for &a in A.iter() {
        let mut a = a;
        while a > 1 {
            let i = osa[a];
            ps.insert(i);

            while a % i == 0 {
                a /= i
            }
        }
    }

    for &p in ps.iter() {
        let mut j = p;
        while j <= M {
            is_ans[j] = false;
            j += p;
        }
    }

    let mut ans = Vec::new();
    ans.push(1);

    for i in 2..=M {
        if is_ans[i] {
            ans.push(i);
        }
    }

    println!("{}", ans.len());
    for i in ans {
        println!("{i}");
    }
}
