#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        N: usize,
        S: [Chars; N],
    }

    let f = |c: char| match c {
        'a' | 'b' | 'c' => '2',
        'd' | 'e' | 'f' => '3',
        'g' | 'h' | 'i' => '4',
        'j' | 'k' | 'l' => '5',
        'm' | 'n' | 'o' => '6',
        'p' | 'q' | 'r' | 's' => '7',
        't' | 'u' | 'v' => '8',
        'w' | 'x' | 'y' | 'z' => '9',
        _ => {
            panic!()
        }
    };

    let mut ans: String = S.iter().map(|s| f(s[0])).join("");
    println!("{}", ans);
}
