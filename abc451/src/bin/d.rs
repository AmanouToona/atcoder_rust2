#![allow(non_snake_case)]
use proconio::input;

fn dfs(n: usize, pow2: &[(usize, usize)], nums: &mut Vec<usize>, max: usize) {
    for &m in pow2.iter() {
        let nxt = n * 10usize.pow(m.1 as u32) + m.0;
        if nxt > max {
            continue;
        }
        nums.push(nxt);
        dfs(nxt, pow2, nums, max);
    }
}

fn main() {
    input! {
        N: usize
    }

    let MAX = 10usize.pow(9_u32);
    let mut pow2: Vec<(usize, usize)> = Vec::new();
    for i in 0..=30 {
        let n = 2usize.pow(i);
        let c = n.to_string().len();
        pow2.push((n, c));
    }

    let mut num = Vec::new();

    dfs(0, &pow2, &mut num, MAX);
    num.sort();
    num.dedup();

    println!("{}", num[N - 1]);

    // eprintln!("{:?}", num);
}
