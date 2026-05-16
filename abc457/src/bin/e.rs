#![allow(non_snake_case)]
use ac_library::Monoid;
use ac_library::Segtree;
use proconio::input;
use std::collections::BTreeSet;

/*
s-t が布で覆われるという条件から、
Lを端とする布を1枚以上
Rを端とする布を1枚以上利用することが確定

L, R を管理する?
L の配列に R の BinaryTree を持たせて t を超えない範囲で最大のもの
R の配列に L の ... を持たせて s 以上の最小のものを引っ張ってくる

これで被覆できるならば解あり。できないならば解なし。

これが同一の布であるならば、解なしになる点に注意。　ちょうど2枚の布を用いることが条件。
-> s-t = l-r となる布が存在する場合のみ場合のみ別

s-t = l-r となる布が存在した場合
完全に s-t に含まれる布が 2枚以上存在することが必要
包除原理で解ける?

(s をまたぐ布の数 + t をまたぐ布の数 - s,t を共に跨ぐ布の数) が条件外

うまくいかないので、ソートして問題を解く順番を変える？
l の昇順かつ r の昇順にソート ... 布、st の両方 ...　カウントはうまく機能しそう

*/

fn main() {
    input! {
        (N, M): (usize, usize),
        mut lr: [(usize, usize); M],
        Q: usize,
        st: [(usize, usize); Q],
    }

    lr.sort_by_key(|x| x.0);
    lr.sort_by_key(|x| x.1);

    let mut st: Vec<(usize, usize, usize)> = st
        .into_iter()
        .enumerate()
        .map(|(i, (s, t))| (s, t, i))
        .collect();

    st.sort_by_key(|x| x.0);
    st.sort_by_key(|x| x.1);

    struct O;
    impl Monoid for O {
        type S = usize;
        fn identity() -> Self::S {
            0
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            a + b
        }
    }

    let mut include_cnt = vec![0; Q];
    let mut left = Segtree::<O>::new(N + 1);
    let mut iter_lr = lr.iter().peekable();
    for &(s, t, i) in st.iter() {
        while let Some(&&(l, r)) = iter_lr.peek() {
            if r <= t {
                left.set(l, left.get(l) + 1);
                iter_lr.next();
            } else {
                break;
            }
        }

        include_cnt[i] = left.prod(s..=t);
    }

    let mut right = vec![BTreeSet::new(); N + 1];
    let mut left = vec![BTreeSet::new(); N + 1];
    for &(l, r) in lr.iter() {
        left[r].insert(l);
        right[l].insert(r);
    }

    let mut ans = vec![false; Q];
    for &(s, t, i) in st.iter() {
        if right[s].range(s..=t).next_back().is_some() && left[t].range(s..=t).next().is_some() {
            let r = *right[s].range(s..=t).next_back().unwrap();
            let l = *left[t].range(s..=t).next().unwrap();

            if l <= r + 1 && include_cnt[i] > 1 {
                ans[i] = true;
            }
        }
    }

    for &i in ans.iter() {
        if i {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
