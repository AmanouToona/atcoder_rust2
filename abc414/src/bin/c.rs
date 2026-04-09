#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        A: usize,
        N: usize,
    }

    let mut ans = 0;
    let mut i = 1;
    'outer: loop {
        let i_str = i.to_string();
        let n = i_str
            .chars()
            .chain(i_str.chars().rev().skip(1))
            .join("")
            .parse::<usize>()
            .unwrap();
        if n > N {
            eprintln!("{n}");
            break;
        }

        let mut tmp = n;
        let mut a = Vec::new();
        while tmp > 0 {
            a.push(tmp % A);
            tmp /= A;
        }

        i += 1;

        for (i, j) in a.iter().zip(a.iter().rev()) {
            if i != j {
                continue 'outer;
            }
        }
        ans += n;
    }

    let mut i = 1;
    'outer: loop {
        let i_str = i.to_string();
        let n = i_str
            .chars()
            .chain(i_str.chars().rev())
            .join("")
            .parse::<usize>()
            .unwrap();
        if n > N {
            eprintln!("{n}");
            break;
        }

        let mut tmp = n;
        let mut a = Vec::new();
        while tmp > 0 {
            a.push(tmp % A);
            tmp /= A;
        }

        i += 1;

        for (i, j) in a.iter().zip(a.iter().rev()) {
            if i != j {
                continue 'outer;
            }
        }
        ans += n;
    }
    println!("{ans}");
}
