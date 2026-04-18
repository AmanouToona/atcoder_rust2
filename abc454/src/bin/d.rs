#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;
/*
- 文字のこすうは2nしか変化しない ... 奇数個の違いがある場合は out
- (), (x), (x....x) は変えようがない。固定

固定のもの以外は一致させることができる？
- xx の個数を変えることはできない
- (xx)) みたいなものも変化させることができない

消せるだけ (, ), を消して一致するをか確かめればいいか?
*/

fn canonical(A: &Vec<char>) -> String {
    let mut q = VecDeque::new();
    for &a in A.iter() {
        if a == ')' {
            let n = q.len();
            if n >= 3 && q[n - 1] == 'x' && q[n - 2] == 'x' && q[n - 3] == '(' {
                for _ in 0..3 {
                    q.pop_back();
                }
                for _ in 0..2 {
                    q.push_back('x');
                }
            } else {
                q.push_back(a);
            }
        } else {
            q.push_back(a);
        }
    }

    q.iter().join("")
}

fn main() {
    input! {
        T: usize
    }

    for _ in 0..T {
        input! {
            A: Chars,
            B: Chars,
        }

        let A = canonical(&A);
        let B = canonical(&B);

        if A == B {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
