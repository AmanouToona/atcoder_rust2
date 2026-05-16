#![allow(non_snake_case)]
use ac_library::ModInt998244353 as mint;
use proconio::input;
use proconio::marker::Chars;

/*
同じ文字が続く部分で区切られると考える

長さ l の 1つの区切りのなかで文字列の組み合わせは、
(l + 1)C2

長さの配列の作り方
今確認している文字が前の文字と異なっていれば 新しく 1
同じであれば前の数字に += 1

*/

fn main() {
    input! {
        S: Chars,
    }

    let mut lens: Vec<usize> = Vec::new();
    lens.push(1);

    for (&s1, &s2) in S.iter().zip(S.iter().skip(1)) {
        if s1 != s2 {
            *lens.last_mut().unwrap() += 1;
        } else {
            lens.push(1);
        }
    }

    let mut ans = mint::new(0);
    for &l in lens.iter() {
        ans += (mint::new(l + 1) * mint::new(l)) / mint::new(2);
    }

    println!("{ans}");
}
