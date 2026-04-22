#![allow(non_snake_case)]
use ac_library::Dsu;
use proconio::input;
/*
木の葉の方から、最大値を処理していけば良さそう。
何回利用されるか？　も同時に保持できる。　自分よりも先にある頂点の数なので。
これを2方向から行う？
いやだめだ。 2方向じゃ足りないのか
木dp と主客転倒の雰囲気を感じる

ある特定のノードについて考える。
複数のノードにつながっているとして... 出ていく方と入ってくる方？

一度直線の木を考えてみるとわかりやすいかも?

他のグラフを考える。　そのグラフの中の最大辺が用いられるのは、グラフを最大辺で切った各グループに属する頂点の掛け算。
小さい方から変を繋いで行って、繋がれたグループに属するノードの数の掛け算が、辺の答えに対する重みになる。

*/

fn main() {
    input! {
        N: usize,
        mut uvw:[(usize, usize, usize); N - 1]
    }

    uvw.sort_by_key(|x| x.2);
    let mut dsu = Dsu::new(N);
    let mut ans = 0;
    for &(u, v, w) in uvw.iter() {
        let u = u - 1;
        let v = v - 1;

        ans += w * dsu.size(u) * dsu.size(v);
        dsu.merge(u, v);
    }

    println!("{ans}");
}
