#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;

struct Solver {
    len: Vec<usize>,
    count: Vec<Vec<usize>>,
    cumsum_x: Vec<Vec<usize>>,
    cumsum_y: Vec<Vec<usize>>,
}

impl Solver {
    fn new(x: &[char], y: &[char]) -> Self {
        let mut cumsum_x = vec![vec![0; 26]; x.len() + 1];
        let mut cumsum_y = vec![vec![0; 26]; y.len() + 1];

        for i in 0..x.len() {
            cumsum_x[i + 1] = cumsum_x[i].clone();
            cumsum_x[i + 1][(x[i] as u8 - b'a') as usize] += 1;
        }

        for i in 0..y.len() {
            cumsum_y[i + 1] = cumsum_y[i].clone();
            cumsum_y[i + 1][(y[i] as u8 - b'a') as usize] += 1;
        }

        let mut len = vec![0, x.len(), y.len()];
        let mut count = vec![
            vec![0; 26],
            cumsum_x.last().unwrap().clone(),
            cumsum_y.last().unwrap().clone(),
        ];

        while *len.last().unwrap() <= 10usize.pow(18u32) {
            let i = len.len();
            len.push(len[i - 1] + len[i - 2]);

            let mut nxt_cnt = vec![0; 26];
            for c in 0..26 {
                nxt_cnt[c] += count[i - 1][c] + count[i - 2][c];
            }
            count.push(nxt_cnt);
        }

        eprintln!("len: {:?}", len);

        Solver {
            len,
            count,
            cumsum_x,
            cumsum_y,
        }
    }

    fn solve(&self, k: usize, c: usize) -> usize {
        if k == 0 {
            return 0;
        }

        let mut k = k;

        let mut res = 0;
        let mut idx = self.len.len() - 1;

        while idx > 0 {
            // eprintln!("idx {idx},  k {k} res{res}");
            if idx == 1 {
                res += self.cumsum_x[k][c];
                break;
            }
            if idx == 2 {
                res += self.cumsum_y[k][c];
                break;
            }

            if k > self.len[idx] {
                idx -= 1;
                continue;
            } else if k == self.len[idx] {
                res += self.count[idx][c];
                break;
            } else if k >= self.len[idx - 1] {
                res += self.count[idx - 1][c];
                k -= self.len[idx - 1];
                idx -= 2;
            } else {
                idx -= 1;
            }
        }

        res
    }
}

fn main() {
    input! {
        X: Chars,
        Y: Chars,
        Q: usize,
    }

    let solve = Solver::new(&X, &Y);

    for _ in 0..Q {
        input! {
            (L, R, C):(usize, usize, char),
        }

        let ans = solve.solve(R, (C as u8 - b'a') as usize)
            - solve.solve(L - 1, (C as u8 - b'a') as usize);
        println!("{ans}");
    }
}
