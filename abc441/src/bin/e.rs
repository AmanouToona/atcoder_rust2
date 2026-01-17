use ac_library::Monoid;
use ac_library::Segtree;
use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: Chars,
    }

    struct M;
    impl Monoid for M {
        type S = usize;
        fn identity() -> Self::S {
            0
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            a + b
        }
    }

    let mut cnt: Vec<i64> = Vec::new();
    cnt.push(0);
    for &s in S.iter() {
        if s == 'A' {
            cnt.push(1);
        } else if s == 'B' {
            cnt.push(-1);
        } else {
            cnt.push(0);
        }
    }

    for i in 0..cnt.len() - 1 {
        cnt[i + 1] += cnt[i];
    }

    let &min = cnt.iter().min().unwrap();
    let &max = cnt.iter().max().unwrap();

    let mut seg = Segtree::<M>::new((max - min + 1) as usize);

    for c in cnt.iter() {
        let c = (c - min) as usize;
        let pre = seg.get(c);
        seg.set(c, pre + 1);
    }

    // eprintln!("cnt {:?}", cnt);
    let mut ans = 0;
    for (i, c) in cnt.iter().enumerate() {
        let need = 1 + c;
        if need <= max {
            // eprintln!("i : {i} c {c}, need {need}");
            let need = (need - min) as usize;
            // eprintln!("converted :{need}, {}", seg.prod(need..));
            ans += seg.prod(need..);
        }

        // 後処理
        let c = (c - min) as usize;
        let pre = seg.get(c);
        seg.set(c, pre - 1);
    }

    println!("{ans}");
}
