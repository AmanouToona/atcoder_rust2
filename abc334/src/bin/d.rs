#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        (N, Q): (usize, usize),
        mut R: [usize; N],
    }

    R.sort();
    let mut pref = vec![0; N + 1];
    for (i, &r) in R.iter().enumerate() {
        pref[i + 1] = pref[i] + r;
    }

    for _ in 0..Q {
        input! {X: usize}
        if *pref.last().unwrap() <= X {
            println!("{N}");
            continue;
        }

        let mut ok = 0;
        let mut ng = N;
        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            if pref[mid] <= X {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        println!("{ok}");
    }
}
