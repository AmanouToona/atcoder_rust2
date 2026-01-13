use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        P: [usize; N],
    }

    let P: Vec<usize> = P.iter().map(|x| x - 1).collect();

    let mut cycles = Vec::new();
    let mut used = vec![false; N];
    for &p in P.iter() {
        if used[p] {
            continue;
        }

        let mut nxt = p;
        let mut size = 0;

        while !used[nxt] {
            used[nxt] = true;
            size += 1;
            nxt = P[nxt];
        }

        cycles.push(size);
    }

    let mut ans: i64 = 0;
    for c in cycles {
        ans += c * (c - 1) / 2;
    }

    println!("{ans}")
}
