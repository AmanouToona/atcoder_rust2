#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {T: usize}
    'outer: for _ in 0..T {
        input! {
            N: usize,
            mut S: [usize; N],
        }

        let g = S[N - 1];
        let s = S[0];

        if g <= s * 2 {
            println!("2");
            continue 'outer;
        }

        let mut domino: Vec<usize> = S[1..N - 1].to_vec();
        domino.sort();

        let mut q = Vec::new();
        q.push(s);
        for &d in domino.iter() {
            // ガード
            if d <= *q.last().unwrap() {
                continue;
            }
            if d > q.last().unwrap() * 2 {
                break;
            }
            if q.last().unwrap() * 2 >= g {
                break;
            }

            // 巻き戻し
            while q.len() >= 2 && q[q.len() - 2] * 2 >= d {
                q.pop();
            }

            // 更新
            q.push(d);
        }

        if q.last().unwrap() * 2 >= g {
            println!("{}", q.len() + 1);
        } else {
            println!("-1")
        }
    }
}
