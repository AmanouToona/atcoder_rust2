#![allow(non_snake_case)]
use proconio::input;
fn main() {
    input! {
        N: usize,
        mut hl: [(usize, usize); N],
        Q: usize,
        T: [usize; Q],
    }

    let mut ti: Vec<(usize, usize)> = T.iter().enumerate().map(|(i, &t)| (t, i)).collect();
    ti.sort_by(|x, y| y.0.cmp(&x.0));

    hl.sort_by(|x, y| y.1.cmp(&x.1));

    // eprintln!("{:?}", ti);
    // eprintln!("{:?}", hl);

    let mut hl_iter = hl.iter().peekable();
    let mut ans = vec![0; Q];
    let mut max = 0;

    for &(t, i) in ti.iter() {
        while let Some(&(h, l)) = hl_iter.peek() {
            if l <= &t {
                break;
            }
            max = max.max(*h);
            hl_iter.next();
        }
        ans[i] = max;
    }

    for a in ans.iter() {
        println!("{a}");
    }
}
